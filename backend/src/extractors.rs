use axum::{
    extract::FromRequestParts,
    http::{StatusCode, header::HOST, request::Parts},
};

use crate::constants::LOCALHOST_DOMAIN_WITH_COLON;

pub struct Domain(pub String);

impl<State> FromRequestParts<State> for Domain
where
    State: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _: &State) -> Result<Self, Self::Rejection> {
        let host_value = parts
            .uri
            .authority()
            .map(|host| host.to_string())
            .or(parts.headers.get(HOST).cloned().and_then(|host_header| {
                host_header
                    .to_str()
                    .ok()
                    .map(|header_value| header_value.to_string())
            }))
            .ok_or((StatusCode::BAD_REQUEST, "missing host header".to_string()))?;
        let host_domains: Vec<&str> = host_value.split(".").collect();
        if host_domains.is_empty() {
            Err((StatusCode::BAD_REQUEST, "invalid host".to_string()))
        } else {
            Ok(Domain(host_domains[host_domains.len() - 2..].join(".")))
        }
    }
}

pub struct URIScheme(pub String);

impl<State> FromRequestParts<State> for URIScheme
where
    State: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &State) -> Result<Self, Self::Rejection> {
        let Domain(domain) = Domain::from_request_parts(parts, state).await?;
        let scheme = if domain.starts_with(LOCALHOST_DOMAIN_WITH_COLON) {
            "http://".to_string()
        } else {
            "https://".to_string()
        };
        Ok(URIScheme(scheme))
    }
}
