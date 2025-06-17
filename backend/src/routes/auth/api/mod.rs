use axum::routing::post;
use axum::{Extension, Json, Router};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};
use models::LoginRequest;

use crate::constants::{ACCESS_TOKEN_COOKIE_NAME, REFRESH_TOKEN_COOKIE_NAME};
use crate::error::Error;
use crate::extractors::Domain;
use crate::managers::crypto::CryptoManager;
use crate::managers::db::DbManager;
use crate::managers::redis::RedisManager;
use crate::models::Secret;
use crate::routes::auth::api::constants::CREDENTIALS_DURATION;

mod constants;
mod models;

pub fn create_router() -> Router {
    Router::new().route("/login", post(login))
}

async fn login(
    cookie_jar: CookieJar,
    Domain(domain): Domain,
    crypto_manager: Extension<CryptoManager>,
    Extension(db_manager): Extension<DbManager>,
    Extension(redis_manager): Extension<RedisManager>,
    Json(payload): Json<LoginRequest>,
) -> Result<(), Error> {
    let user_data = db_manager
        .get_user_data(&payload.username)
        .await?
        .ok_or(Error::BadCredentials)?;
    let valid_password =
        crypto_manager.matches(&payload.password_hash, &user_data.password_hash)?;

    if !valid_password {
        Err(Error::BadCredentials)?
    }

    let access_token = Secret::default().get();
    let refresh_token = Secret::default().get();

    redis_manager
        .store_auth_tokens(&access_token, &refresh_token, user_data.id)
        .await?;

    let mut access_token_cookie = auth_cookie(ACCESS_TOKEN_COOKIE_NAME.to_string(), access_token);
    if let Some(domain) = domain.clone() {
        access_token_cookie.set_domain(domain);
    }

    let mut refresh_token_cookie =
        auth_cookie(REFRESH_TOKEN_COOKIE_NAME.to_string(), refresh_token);
    if domain.is_some() {
        refresh_token_cookie.set_path("/api/refresh-credentials");
    } else {
        // This means we are in development environment
        refresh_token_cookie.set_path("/auth/api/refresh-credentials");
    }

    let _ = cookie_jar
        .add(access_token_cookie)
        .add(refresh_token_cookie);

    Ok(())
}

pub fn auth_cookie<'a>(name: String, value: String) -> Cookie<'a> {
    let mut cookie = Cookie::new(name, value);

    cookie.set_http_only(true);
    cookie.set_max_age(Some(CREDENTIALS_DURATION));
    cookie.set_same_site(SameSite::Strict);
    cookie.set_secure(true);

    cookie
}
