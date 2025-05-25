use crate::error::Error;
use crate::logger::Logger;
use crate::server::Server;
use crate::settings::Settings;
use axum::extract::DefaultBodyLimit;
use clap::Parser;
use routes::create_router;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

mod error;
mod logger;
mod middleware;
mod routes;
mod server;
mod settings;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let settings = Settings::parse();

    Logger::new(&settings).init();

    let app = create_router()
        .layer(TraceLayer::new_for_http())
        .layer(DefaultBodyLimit::disable())
        .layer(CorsLayer::very_permissive());

    Server::new(&settings).start(&app).await?;

    Ok(())
}
