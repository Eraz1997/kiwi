use axum::Router;

mod services;
mod users;

pub fn create_router() -> Router {
    Router::new()
        .nest("/services", services::create_router())
        .nest("/users", users::create_router())
}
