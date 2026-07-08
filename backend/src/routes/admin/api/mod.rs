use axum::Router;

use crate::state::AppState;

mod certificates;
mod dynamic_dns;
mod services;
mod users;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/dynamic-dns", dynamic_dns::create_router())
        .nest("/services", services::create_router())
        .nest("/certificates", certificates::create_router())
        .nest("/users", users::create_router())
}
