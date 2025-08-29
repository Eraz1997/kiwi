use std::str::FromStr;

use axum::body::to_bytes;
use axum::extract::Request;
use axum::http::uri::{Authority, Parts, PathAndQuery, Scheme};
use axum::response::Response;
use hyper::Uri;
use reqwest::{Body, Client};

use crate::error::Error;
use crate::settings::Settings;

#[derive(Clone)]
pub struct LocalHttpManager {
    client: Client,
    dev_frontend_base_url: String,
}

impl LocalHttpManager {
    pub fn new(settings: &Settings) -> Result<Self, Error> {
        let client = Client::builder().https_only(false).build()?;
        let dev_frontend_base_url =
            format!("http://localhost:{}", settings.dev_frontend_server_port);

        tracing::info!("local http manager initialised");

        Ok(Self {
            client,
            dev_frontend_base_url,
        })
    }

    pub async fn get_dev_frontend_page(&self, path: String) -> Result<Response<Body>, Error> {
        let url = if path.starts_with("/") {
            format!("{}{}", self.dev_frontend_base_url, path)
        } else {
            format!("{}/{}", self.dev_frontend_base_url, path)
        };
        let response = self.client.get(url).send().await?;
        Ok(response.into())
    }

    pub async fn forward_request(
        &self,
        original_request: Request,
        path: String,
        port: i32,
    ) -> Result<Response<Body>, Error> {
        let (mut parts, body) = original_request.into_parts();
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

        let request = reqwest::Request::try_from(Request::from_parts(parts, body_bytes))?;
        let response = self.client.execute(request).await?;
        Ok(response.into())
    }
}
