use axum::response::Response;
use reqwest::{Body, Client};

use crate::error::Error;
use crate::settings::Settings;

#[derive(Clone)]
pub struct DevFrontendManager {
    client: Client,
    client_base_url: String,
}

impl DevFrontendManager {
    pub fn new(settings: &Settings) -> Result<Self, Error> {
        let client = Client::builder().https_only(false).build()?;
        let client_base_url = format!("http://localhost:{}", settings.dev_frontend_server_port);

        Ok(Self {
            client,
            client_base_url,
        })
    }

    pub async fn get(&self, path: String) -> Result<Response<Body>, Error> {
        let url = if path.starts_with("/") {
            format!("{}{}", self.client_base_url, path)
        } else {
            format!("{}/{}", self.client_base_url, path)
        };
        let response = self.client.get(url).send().await?;
        Ok(response.into())
    }
}
