use crate::error::Error;
use crate::logger::Logger;
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

mod error;
mod logger;
mod managers;
mod middlewares;
mod routes;
mod server;
mod settings;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let settings = Settings::parse();

    Logger::new(&settings).init();

    let secrets_manager = SecretsManager::new_with_loaded_or_created_secrets().await?;
    let container_manager = ContainerManager::new().await?;

    let db_admin_username = secrets_manager.db_admin_username();
    let db_admin_password = secrets_manager.db_admin_password();

    let db_container_configuration =
        ContainerConfiguration::get_postgres_configuration(&db_admin_username, &db_admin_password)?;

    container_manager
        .start_container(&db_container_configuration)
        .await?;

    let db_manager = DbManager::new(&db_admin_username, &db_admin_password).await?;

    let app = create_router()
        .layer(TraceLayer::new_for_http())
        .layer(DefaultBodyLimit::disable())
        .layer(CorsLayer::very_permissive())
        .layer(middleware::from_fn(authentication_middleware))
        .layer(Extension(db_manager))
        .layer(Extension(container_manager));

    Server::new(&settings).start(&app).await?;

    Ok(())
}
