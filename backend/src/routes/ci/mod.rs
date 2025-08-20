use axum::Router;

mod api;

pub fn create_router() -> Router {
    Router::new().nest("/api", api::create_router())
}
