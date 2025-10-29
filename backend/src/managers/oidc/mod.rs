use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use reqwest::Client;

use crate::error::Error;
use crate::managers::oidc::models::{GithubClaims, GithubJwksResponse, Jwk};

mod error;
mod models;

#[derive(Clone)]
pub struct OidcManager {
    github_jwks: Vec<Jwk>,
}

impl OidcManager {
    pub async fn new() -> Result<Self, Error> {
        let client = Client::new();

        let jwks: GithubJwksResponse = client
            .get("https://token.actions.githubusercontent.com/.well-known/jwks")
            .send()
            .await?
            .json()
            .await?;

        tracing::info!(
            "initialised oidc manager with {} github jwks",
            jwks.keys.len()
        );

        Ok(Self {
            github_jwks: jwks.keys,
        })
    }

    pub async fn validate_github_oidc_token(&self, token: &str) -> Result<GithubClaims, Error> {
        let kid = decode_header(token)
            .map_err(|_| Error::invalid_header())?
            .kid
            .ok_or(Error::invalid_key_id())?;

        let jwk = self
            .github_jwks
            .iter()
            .find(|k| k.kid == kid)
            .ok_or(Error::invalid_key_id())?;

        let modulus = jwk.n.as_str();
        let exponent = jwk.e.as_str();

        let decoding_key = DecodingKey::from_rsa_components(modulus, exponent)
            .map_err(|_| Error::invalid_key())?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&["kiwiDeploy"]);

        let claims = decode::<GithubClaims>(token, &decoding_key, &validation)
            .map_err(|_| Error::invalid_jwt())?
            .claims;

        Ok(claims)
    }
}
