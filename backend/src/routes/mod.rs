use axum::Router;

pub mod auth;

static AUTH_PATH_PREFIX: &str = "auth";
pub static ALL_PATH_PREFIXES: &[&str] = &[AUTH_PATH_PREFIX];

pub fn create_router() -> Router {
    Router::new().nest(
        format!("/{}", AUTH_PATH_PREFIX).as_str(),
        auth::create_router(),
    )
}
