use axum::routing::{get, post};
use axum::{Json, Router};
use models::LoginRequest;

mod models;

pub fn create_router() -> Router {
    Router::new()
        .route("/healthy", get(healthy))
        .route("/login", post(login))
}

async fn healthy() -> &'static str {
    "OK"
}

async fn login(Json(_): Json<LoginRequest>) {}
