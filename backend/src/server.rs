use crate::{middlewares::subdomain::subdomain_middleware, settings::Settings};
use axum::Router;
use axum::ServiceExt;
use axum_server::bind;
use axum_server::bind_rustls;
use axum_server::tls_rustls::RustlsConfig;
use std::io;
use std::net::{SocketAddr, ToSocketAddrs};
use std::path::PathBuf;
use tokio::fs::File;
use tower::Layer;
use tower::util::MapRequestLayer;

pub struct Server {
    connection_string: String,
    is_development: bool,
    tls_public_certificate_path: String,
    tls_private_key_path: String,
}

impl Server {
    pub fn new(settings: &Settings) -> Self {
        Self {
            connection_string: settings.connection_string(),
            is_development: settings.is_development(),
            tls_public_certificate_path: settings.tls_public_certificate_path(),
            tls_private_key_path: settings.tls_private_key_path(),
        }
    }

    pub async fn start(&self, app: &Router) -> Result<(), io::Error> {
        let subdomain_handler_middleware = MapRequestLayer::new(subdomain_middleware);
        let app_with_middlewares = subdomain_handler_middleware.layer(app.clone());
        let socket_addresses: Vec<SocketAddr> = self.connection_string.to_socket_addrs()?.collect();
        let socket_address = *socket_addresses.first().ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            "server address invalid",
        ))?;

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

        if File::open(&self.tls_public_certificate_path).await.is_err()
            || File::open(&self.tls_private_key_path).await.is_err()
        {
            bind(socket_address)
                .serve(app_with_middlewares.into_make_service())
                .await
        } else {
            let tls_config = RustlsConfig::from_pem_file(
                PathBuf::from(self.tls_public_certificate_path.clone()),
                PathBuf::from(self.tls_private_key_path.clone()),
            )
            .await?;

            bind_rustls(socket_address, tls_config)
                .serve(app_with_middlewares.into_make_service())
                .await
        }
    }
}
