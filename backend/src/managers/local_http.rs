use std::str::FromStr;

use axum::body::to_bytes;
use axum::extract::Request;
use axum::response::Response;
use reqwest::{Body, Client, Url};

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
        let url = if path.starts_with("/") {
            format!("http://localhost:{}{}", port, path)
        } else {
            format!("http://localhost:{}/{}", port, path)
        };

        let (parts, body) = original_request.into_parts();
        let body_bytes = to_bytes(body, usize::MAX)
            .await
            .map_err(|_| Error::serialisation())?;

        let mut request = reqwest::Request::try_from(Request::from_parts(parts, body_bytes))?;
        *request.url_mut() = Url::from_str(url.as_str()).map_err(|_| Error::serialisation())?;
        let response = self.client.execute(request).await?;
        Ok(response.into())
    }
}
