use axum::Router;
use axum::routing::get;

pub fn create_router() -> Router {
    Router::new().route("/healthy", get(healthy))
}

async fn healthy() -> &'static str {
    "OK"
}
