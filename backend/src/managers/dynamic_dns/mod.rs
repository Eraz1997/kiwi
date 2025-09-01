use reqwest::{
    Client,
    header::{AUTHORIZATION, HeaderMap, HeaderValue},
};

use crate::{
    error::Error,
    managers::{
        dynamic_dns::models::DnsRecordValue,
        secrets::models::{DynamicDnsApiConfiguration, DynamicDnsProvider},
    },
};

mod error;
mod models;

#[derive(Clone)]
pub struct DynamicDnsManager {
    dynamic_dns_provider_client: Client,
    dynamic_dns_provider_update_uri: String,
    public_ip_retriever_client: Client,
    current_ip_address: Option<String>,
}

impl DynamicDnsManager {
    pub async fn new(
        dynamic_dns_api_configuration: &DynamicDnsApiConfiguration,
    ) -> Result<Self, Error> {
        let mut auth_headers = HeaderMap::new();
        auth_headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&dynamic_dns_api_configuration.authorization_header.get())
                .map_err(|_| Error::serialisation())?,
        );

        let dynamic_dns_provider_client =
            Client::builder().default_headers(auth_headers).build()?;
        let dynamic_dns_provider_update_uri = match dynamic_dns_api_configuration.provider {
            DynamicDnsProvider::GoDaddy => format!(
                "https://api.godaddy.com/v1/domains/{}/records/A/*",
                dynamic_dns_api_configuration.domain.get()
            ),
        };

        let test_uri = match dynamic_dns_api_configuration.provider {
            DynamicDnsProvider::GoDaddy => "https://api.godaddy.com/v1/domains".to_string(),
        };

        dynamic_dns_provider_client
            .get(test_uri)
            .send()
            .await
            .map_err(|_| Error::provider_test_failed())?
            .error_for_status()
            .map_err(|_| Error::provider_test_failed())?;

        let public_ip_retriever_client = Client::builder().build()?;

        tracing::info!(
            "dynamic dns manager initialised with provider {}",
            dynamic_dns_api_configuration.provider
        );

        Ok(Self {
            dynamic_dns_provider_client,
            dynamic_dns_provider_update_uri,
            public_ip_retriever_client,
            current_ip_address: None,
        })
    }

    pub async fn refresh(&mut self) -> Result<(), Error> {
        let new_ip_address = self
            .public_ip_retriever_client
            .get("https://api.ipify.org?format=text")
            .send()
            .await?
            .text()
            .await?;

        if Some(new_ip_address.clone()) != self.current_ip_address {
            let dns_records = vec![DnsRecordValue {
                data: new_ip_address.clone(),
            }];
            self.dynamic_dns_provider_client
                .put(&self.dynamic_dns_provider_update_uri)
                .json(&dns_records)
                .send()
                .await?;

            tracing::info!("dynamic dns records refreshed");
        }

        self.current_ip_address = Some(new_ip_address);

        Ok(())
    }
}
