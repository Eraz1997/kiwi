use axum::{
    Extension, Router,
    extract::{Path, Request},
    response::Response,
    routing::any,
};
use reqwest::Body;

use crate::{
    error::Error,
    managers::{db::DbManager, local_http::LocalHttpManager, redis::RedisManager},
    settings::Settings,
};

pub mod admin;
pub mod auth;
pub mod ci;
mod error;

pub fn create_router(settings: &Settings) -> Router {
    Router::new()
        .nest("/admin", admin::create_router(settings))
        .nest("/auth", auth::create_router(settings))
        .nest("/ci", ci::create_router())
        .route("/{service}/{*path}", any(forward_to_service))
}

async fn forward_to_service(
    Extension(redis_manager): Extension<RedisManager>,
    Extension(db_manager): Extension<DbManager>,
    Extension(local_http_manager): Extension<LocalHttpManager>,
    Path((service, path)): Path<(String, String)>,
    request: Request,
) -> Result<Response<Body>, Error> {
    let service_port = redis_manager.get_service_port(&service).await?;

    let port = if let Some(port) = service_port.port {
        Some(port)
    } else {
        let port = db_manager.get_service_port(&service).await?;
        if let Some(port) = port {
            redis_manager.store_service_port(&service, port).await?;
        }
        port
    };

    match port {
        Some(port) => {
            local_http_manager
                .forward_request(request, path, port)
                .await
        }
        None => Err(Error::service_not_found()),
    }
}
