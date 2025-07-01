use crate::{middlewares::subdomain::subdomain_middleware, settings::Settings};
use axum::ServiceExt;
use axum::{Router, serve};
use std::io;
use tokio::net::TcpListener;
use tower::Layer;
use tower::util::MapRequestLayer;

pub struct Server {
    connection_string: String,
    is_development: bool,
}

impl Server {
    pub fn new(settings: &Settings) -> Self {
        Self {
            connection_string: settings.connection_string(),
            is_development: settings.is_development(),
        }
    }

    pub async fn start(&self, app: &Router) -> Result<(), io::Error> {
        let listener = TcpListener::bind(self.connection_string.clone()).await?;
        let environment = if self.is_development {
            "development".to_string()
        } else {
            "production".to_string()
        };

        tracing::info!(
            "{} server listening on {}",
            environment,
            self.connection_string
        );

        let subdomain_handler_middleware = MapRequestLayer::new(subdomain_middleware);
        let app_with_middlewares = subdomain_handler_middleware.layer(app.clone());
        serve(listener, app_with_middlewares.into_make_service()).await
    }
}
