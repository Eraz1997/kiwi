use std::sync::Arc;

use crate::error::Error;
use crate::logger::Logger;
use crate::managers::crypto::CryptoManager;
use crate::managers::dynamic_dns::DynamicDnsManager;
use crate::managers::lets_encrypt::LetsEncryptManager;
use crate::managers::local_http::LocalHttpManager;
use crate::managers::oidc::OidcManager;
use crate::managers::redis::RedisManager;
use crate::managers::secrets::SecretsManager;
use crate::models::ServerAction;
use crate::server::Server;
use crate::settings::Settings;
use crate::worker::Worker;
use axum::extract::DefaultBodyLimit;
use axum::{Extension, middleware};
use clap::Parser;
use managers::container::ContainerManager;
use managers::container::models::ContainerConfiguration;
use managers::db::DbManager;
use middlewares::authentication::authentication_middleware;
use routes::create_router;
use tokio::select;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

mod constants;
mod error;
mod extractors;
mod logger;
mod managers;
mod middlewares;
mod models;
mod routes;
mod server;
mod services;
mod settings;
mod worker;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let settings = Settings::parse();

    Logger::new(&settings).init();

    let mut secrets_manager = SecretsManager::new_with_loaded_or_created_secrets(&settings).await?;
    let container_manager = ContainerManager::new().await?;
    let oidc_manager = OidcManager::new().await?;

    let crypto_pepper = secrets_manager.crypto_pepper();
    let db_admin_username = secrets_manager.db_admin_username();
    let db_admin_password = secrets_manager.db_admin_password();
    let redis_admin_password = secrets_manager.redis_admin_password();

    let crypto_manager = CryptoManager::new(&crypto_pepper)?;

    let db_container_configuration =
        ContainerConfiguration::get_postgres_configuration(&db_admin_username, &db_admin_password)?;
    let redis_container_configuration =
        ContainerConfiguration::get_redis_configuration(&redis_admin_password)?;

    for container_configuration in [db_container_configuration, redis_container_configuration] {
        container_manager
            .start_container(&container_configuration)
            .await?;
    }

    let db_manager = DbManager::new(&db_admin_username, &db_admin_password).await?;
    let redis_manager = RedisManager::new(&redis_admin_password).await?;
    let local_http_manager = LocalHttpManager::new(&settings)?;
    let dynamic_dns_manager = match secrets_manager.dynamic_dns_api_configuration() {
        Some(configuration) => Arc::new(Mutex::new(Some(
            DynamicDnsManager::new(&configuration).await?,
        ))),
        None => Arc::new(Mutex::new(None)),
    };
    let lets_encrypt_manager = Arc::new(Mutex::new(
        LetsEncryptManager::new(
            &settings.lets_encrypt_directory_url(),
            secrets_manager.lets_encrypt_credentials(),
            settings.tls_private_key_path(),
            settings.tls_public_certificate_path(),
        )
        .await?,
    ));

    let services = db_manager.get_services_data().await?;
    for service in services {
        container_manager
            .start_container(&service.container_configuration)
            .await?;
        container_manager
            .create_and_attach_network_for_container(&service.container_configuration)
            .await?;
    }
    secrets_manager
        .set_lets_encrypt_credentials(lets_encrypt_manager.lock().await.get_credentials())
        .await?;

    let invitation = db_manager
        .get_or_create_admin_invitation_if_no_admin_yet()
        .await?;
    if let Some(invitation) = invitation {
        tracing::warn!(
            "admin user not found. invitation created with ID: {}. please visit https://auth.<your-domain>/create-user?invitation_id={}",
            invitation.id,
            invitation.id
        );
    }

    let app = create_router(&settings)
        .layer(TraceLayer::new_for_http())
        .layer(DefaultBodyLimit::disable())
        .layer(CorsLayer::very_permissive())
        .layer(middleware::from_fn(authentication_middleware))
        .layer(Extension(db_manager))
        .layer(Extension(container_manager))
        .layer(Extension(crypto_manager))
        .layer(Extension(redis_manager))
        .layer(Extension(local_http_manager))
        .layer(Extension(oidc_manager))
        .layer(Extension(dynamic_dns_manager.clone()))
        .layer(Extension(Arc::new(Mutex::new(secrets_manager))))
        .layer(Extension(lets_encrypt_manager.clone()));

    let server = Server::new(&settings);
    let worker = Worker::new(dynamic_dns_manager, lets_encrypt_manager);

    loop {
        let server_action = select! {
            _ = server.start(&app) => ServerAction::CloseDueToUnexpectedError,
            server_action = worker.start() => server_action,
        };

        match server_action {
            ServerAction::RestartWithoutDependenciesInit => {}
            ServerAction::CloseDueToUnexpectedError => {
                return Err(Error::unexpected_close());
            }
        }
    }
}
