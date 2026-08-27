use axum::{Router, routing::get};
use kangaroo_axum::{KangarooConfig, KangarooRouterExtension, kangarooise};

use crate::state::AppState;

mod api;

pub fn create_router(kangaroo_config: &KangarooConfig) -> Router<AppState> {
    Router::new()
        .nest("/api", api::create_router())
        .route("/create-user", get(get_create_user))
        .route("/login", get(get_login))
        .route("/logout", get(get_logout))
        .with_kangaroo(kangaroo_config.clone())
}

#[kangarooise]
async fn get_create_user() {}

#[kangarooise]
async fn get_login() {}

#[kangarooise]
async fn get_logout() {}
