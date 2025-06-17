use axum::{
    extract::FromRequestParts,
    http::{HeaderValue, StatusCode, header::HOST, request::Parts},
};

pub struct Domain(pub Option<String>);

impl<State> FromRequestParts<State> for Domain
where
    State: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _: &State) -> Result<Self, Self::Rejection> {
        let host_header = parts
            .headers
            .get(HOST)
            .cloned()
            .unwrap_or(HeaderValue::from_static(""));
        let host_value = host_header.to_str().map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "string conversion error".to_string(),
            )
        })?;
        let host_domains: Vec<&str> = host_value.split(".").collect();
        if host_domains.len() > 1 {
            Ok(Domain(Some(
                host_domains[host_domains.len() - 2..].join("."),
            )))
        } else {
            Ok(Domain(None))
        }
    }
}
