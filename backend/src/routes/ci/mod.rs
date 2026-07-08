use axum::Router;

use crate::state::AppState;

mod api;

pub fn create_router() -> Router<AppState> {
    Router::new().nest("/api", api::create_router())
}
