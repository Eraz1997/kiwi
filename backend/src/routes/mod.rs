use axum::{Router, extract::Path, routing::any};

use crate::settings::Settings;

pub mod admin;
pub mod auth;

pub fn create_router(settings: &Settings) -> Router {
    Router::new()
        .nest("/admin", admin::create_router())
        .nest("/auth", auth::create_router(settings))
        .route("/{service}/{*path}", any(forward_to_service))
}

async fn forward_to_service(Path((service, path)): Path<(String, String)>) {
    todo!("forward to {} with path {}", service, path)
}
