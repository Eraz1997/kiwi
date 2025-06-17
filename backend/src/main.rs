use crate::error::Error;
use crate::logger::Logger;
use crate::managers::crypto::CryptoManager;
use crate::managers::redis::RedisManager;
use crate::managers::secrets::SecretsManager;
use crate::server::Server;
use crate::settings::Settings;
use axum::extract::DefaultBodyLimit;
use axum::{Extension, middleware};
use clap::Parser;
use managers::container::ContainerManager;
use managers::container::models::ContainerConfiguration;
use managers::db::DbManager;
use middlewares::authentication::authentication_middleware;
use routes::create_router;
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
mod settings;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let settings = Settings::parse();

    Logger::new(&settings).init();

    let secrets_manager = SecretsManager::new_with_loaded_or_created_secrets().await?;
    let container_manager = ContainerManager::new().await?;

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

    let app = create_router()
        .layer(TraceLayer::new_for_http())
        .layer(DefaultBodyLimit::disable())
        .layer(CorsLayer::very_permissive())
        .layer(middleware::from_fn(authentication_middleware))
        .layer(Extension(db_manager))
        .layer(Extension(container_manager))
        .layer(Extension(crypto_manager))
        .layer(Extension(redis_manager));

    Server::new(&settings).start(&app).await?;

    Ok(())
}
