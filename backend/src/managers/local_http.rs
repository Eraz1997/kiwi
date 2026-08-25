use std::str::FromStr;

use axum::body::to_bytes;
use axum::extract::Request;
use axum::http::HeaderValue;
use axum::http::header::COOKIE;
use axum::http::uri::{Authority, Parts, PathAndQuery, Scheme};
use axum::response::Response;
use hyper::Uri;
use reqwest::{Body, Client, Version};

use crate::constants::{
    ACCESS_TOKEN_COOKIE_NAME, LOGOUT_REFRESH_TOKEN_COPY_NAME, REFRESH_TOKEN_COOKIE_NAME,
};
use crate::error::Error;

#[derive(Clone)]
pub struct LocalHttpManager {
    client: Client,
}

impl LocalHttpManager {
    pub fn new() -> Result<Self, Error> {
        let client = Client::builder().https_only(false).build()?;

        tracing::info!("local http manager initialised");

        Ok(Self { client })
    }

    pub async fn forward_request(
        &self,
        original_request: Request,
        path: String,
        port: i32,
    ) -> Result<Response<Body>, Error> {
        let (mut parts, body) = original_request.into_parts();

        // strip authentication cookies
        if let Some(cookie_header_value) = parts.headers.get(COOKIE) {
            let cookies = cookie_header_value
                .to_str()
                .map_err(|_| Error::serialisation())?;

            let stripped_cookies = cookies
                .split(';')
                .filter(|cookie_pair| {
                    let (name, _) = cookie_pair.split_once('=').unwrap_or((cookie_pair, ""));
                    let name = name.trim();
                    name != ACCESS_TOKEN_COOKIE_NAME
                        && name != REFRESH_TOKEN_COOKIE_NAME
                        && name != LOGOUT_REFRESH_TOKEN_COPY_NAME
                })
                .collect::<Vec<&str>>()
                .join(";");

            parts.headers.insert(
                COOKIE,
                HeaderValue::from_str(&stripped_cookies).map_err(|_| Error::serialisation())?,
            );
        }

        let sanitised_path = if path.starts_with("/") {
            path
        } else {
            format!("/{}", path)
        };

        let mut uri_parts = Parts::default();
        let authority = format!("localhost:{}", port);
        uri_parts.scheme = Some(Scheme::HTTP);
        uri_parts.authority =
            Some(Authority::from_str(&authority).map_err(|_| Error::serialisation())?);
        uri_parts.path_and_query =
            Some(PathAndQuery::from_str(&sanitised_path).map_err(|_| Error::serialisation())?);

        parts.uri = Uri::from_parts(uri_parts).map_err(|_| Error::serialisation())?;

        let body_bytes = to_bytes(body, usize::MAX)
            .await
            .map_err(|_| Error::serialisation())?;

        let mut request = reqwest::Request::try_from(Request::from_parts(parts, body_bytes))?;
        *request.version_mut() = Version::HTTP_11;
        let response = self.client.execute(request).await?;
        Ok(response.into())
    }
}
