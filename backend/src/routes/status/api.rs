use axum::{Router, routing::get};

use crate::state::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/health", get(check_health))
}

async fn check_health() {}
