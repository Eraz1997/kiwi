use std::time::SystemTime;

use chrono::DateTime;
use instant_acme::{
    Account, AccountCredentials, AuthorizationStatus, ChallengeType, Identifier, NewAccount,
    NewOrder, OrderStatus, RetryPolicy,
};
use rcgen::generate_simple_self_signed;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};
use x509_parser::{
    pem::Pem,
    prelude::{FromDer, X509Certificate},
};

use crate::{
    error::Error,
    managers::lets_encrypt::models::{
        CertificateInfo, CertificateVerificationStatus, NewCertificateOrder,
    },
};

mod error;
pub mod models;

#[derive(Clone)]
pub struct LetsEncryptManager {
    account: Account,
    serialised_credentials: String,
    tls_private_key_path: String,
    tls_public_certificate_path: String,
    tls_private_key_last_modified_date: SystemTime,
}

impl LetsEncryptManager {
    pub async fn new(
        directory_url: &str,
        credentials: Option<String>,
        tls_private_key_path: String,
        tls_public_certificate_path: String,
    ) -> Result<Self, Error> {
        let (account, serialised_credentials) = match &credentials {
            Some(serialised_credentials) => {
                let credentials: AccountCredentials = serde_json::from_str(serialised_credentials)?;
                let account = Account::builder()?.from_credentials(credentials).await?;
                (account, serialised_credentials.to_owned())
            }
            None => {
                let (account, credentials) = Account::builder()?
                    .create(
                        &NewAccount {
                            contact: &[],
                            terms_of_service_agreed: true,
                            only_return_existing: false,
                        },
                        directory_url.to_string(),
                        None,
                    )
                    .await?;

                let serialised_credentials = serde_json::to_string(&credentials)?;
                (account, serialised_credentials)
            }
        };

        if credentials.is_some() {
            tracing::info!("let's encrypt manager initialised with existing credentials");
        } else {
            tracing::info!("let's encrypt manager initialised with new credentials");
        }

        if File::open(&tls_public_certificate_path).await.is_err()
            || File::open(&tls_private_key_path).await.is_err()
        {
            let dummy_certificate_domain = vec!["localhost".to_string()];
            let dummy_certificate = generate_simple_self_signed(dummy_certificate_domain)?;

            let mut tls_private_key_file = File::create(&tls_private_key_path).await?;
            tls_private_key_file
                .write_all(dummy_certificate.signing_key.serialize_pem().as_bytes())
                .await?;

            let mut tls_public_certificate_file =
                File::create(&tls_public_certificate_path).await?;
            tls_public_certificate_file
                .write_all(dummy_certificate.cert.pem().as_bytes())
                .await?;

            tracing::warn!("tls certificate not found, dummy certificate generated");
        }

        let tls_private_key_last_modified_date = File::open(&tls_private_key_path)
            .await?
            .metadata()
            .await?
            .modified()?;

        Ok(LetsEncryptManager {
            account,
            serialised_credentials,
            tls_private_key_path,
            tls_public_certificate_path,
            tls_private_key_last_modified_date,
        })
    }

    pub fn get_credentials(&self) -> String {
        self.serialised_credentials.clone()
    }

    pub async fn order_new_certificate(&self, domain: &str) -> Result<NewCertificateOrder, Error> {
        let wildcard_domain = format!("*.{}", domain);
        let identifiers = vec![Identifier::Dns(wildcard_domain)];
        let mut order = self.account.new_order(&NewOrder::new(&identifiers)).await?;

        if order.state().status != OrderStatus::Pending {
            return Err(Error::bad_order_status());
        }

        let order_url = order.url().to_string();
        let mut authorisations = order.authorizations();
        let mut authorisation = authorisations
            .next()
            .await
            .ok_or(Error::cannot_find_authorisation())??;

        let new_certificate_order = match authorisation.status {
            AuthorizationStatus::Pending => {
                let challenge = authorisation
                    .challenge(ChallengeType::Dns01)
                    .ok_or(Error::bad_authorisation_status())?;
                NewCertificateOrder {
                    order_url,
                    dns_record_name: format!("_acme-challenge.{}", challenge.identifier())
                        .replace("*.", ""),
                    dns_record_value: challenge.key_authorization().dns_value(),
                }
            }
            _ => return Err(Error::bad_authorisation_status()),
        };

        Ok(new_certificate_order)
    }

    pub async fn finalise_and_save_certificates(
        &self,
        order_url: &str,
    ) -> Result<CertificateVerificationStatus, Error> {
        let mut order = self.account.order(order_url.to_string()).await?;
        let mut authorisations = order.authorizations();
        let mut authorisation = authorisations
            .next()
            .await
            .ok_or(Error::cannot_find_authorisation())??;

        let certificate_status = match authorisation.status {
            AuthorizationStatus::Pending => {
                let mut challenge = authorisation
                    .challenge(ChallengeType::Dns01)
                    .ok_or(Error::bad_authorisation_status())?;
                challenge.set_ready().await?;
                CertificateVerificationStatus::Pending
            }
            AuthorizationStatus::Valid => {
                let tls_private_key = order.finalize().await?;
                let tls_public_certificate =
                    order.poll_certificate(&RetryPolicy::default()).await?;

                let mut tls_private_key_file = File::create(&self.tls_private_key_path).await?;
                tls_private_key_file
                    .write_all(tls_private_key.as_bytes())
                    .await?;

                let mut tls_public_certificate_file =
                    File::create(&self.tls_public_certificate_path).await?;
                tls_public_certificate_file
                    .write_all(tls_public_certificate.as_bytes())
                    .await?;

                CertificateVerificationStatus::Success
            }
            _ => CertificateVerificationStatus::Error,
        };

        Ok(certificate_status)
    }

    pub async fn was_certificate_updated(&mut self) -> Result<bool, Error> {
        let tls_private_key_last_modified_date = File::open(&self.tls_private_key_path)
            .await?
            .metadata()
            .await?
            .modified()?;
        let was_updated =
            tls_private_key_last_modified_date > self.tls_private_key_last_modified_date;

        self.tls_private_key_last_modified_date = tls_private_key_last_modified_date;

        Ok(was_updated)
    }

    pub async fn get_certificate_info(&self) -> Result<CertificateInfo, Error> {
        let mut file = File::open(&self.tls_public_certificate_path).await?;
        let mut certificate_bytes = vec![];
        file.read_to_end(&mut certificate_bytes).await?;

        let (pem, _) = Pem::read(std::io::Cursor::new(&certificate_bytes))?;
        let (_, certificate_info) = X509Certificate::from_der(&pem.contents)?;

        let issuer = certificate_info.issuer().to_string();
        let expiration_date =
            DateTime::from_timestamp(certificate_info.validity().not_after.timestamp(), 0)
                .ok_or(Error::serialisation())?
                .naive_utc();

        Ok(CertificateInfo {
            issuer,
            expiration_date,
        })
    }
}
