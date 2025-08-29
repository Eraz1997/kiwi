use axum::Router;

mod dynamic_dns;
mod services;
mod users;

pub fn create_router() -> Router {
    Router::new()
        .nest("/dynamic-dns", dynamic_dns::create_router())
        .nest("/services", services::create_router())
        .nest("/users", users::create_router())
}
