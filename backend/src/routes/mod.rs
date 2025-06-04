use axum::{Router, extract::Path, routing::any};

pub mod admin;
pub mod auth;

pub fn create_router() -> Router {
    Router::new()
        .nest("/admin", admin::create_router())
        .nest("/auth", auth::create_router())
        .route("/{service}/{*path}", any(forward_to_service))
}

async fn forward_to_service(Path((service, path)): Path<(String, String)>) {
    todo!("forward to {} with path {}", service, path)
}
