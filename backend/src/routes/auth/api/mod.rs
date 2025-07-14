use axum::extract::Query;
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::{any, get, post};
use axum::{Extension, Json, Router};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};
use models::LoginRequest;
use time::Duration;
use urlencoding::decode;

use crate::constants::{ACCESS_TOKEN_COOKIE_NAME, REFRESH_TOKEN_COOKIE_NAME};
use crate::error::Error;
use crate::extractors::{Domain, URIScheme};
use crate::managers::crypto::CryptoManager;
use crate::managers::db::DbManager;
use crate::managers::redis::RedisManager;
use crate::managers::redis::models::{RedisRefreshToken, RedisRefreshTokenKind};
use crate::models::{Secret, UserRole};
use crate::routes::auth::api::constants::CREDENTIALS_DURATION;
use crate::routes::auth::api::models::{
    CreateUserRequest, GetSealingKeyResponse, RefreshCredentialsQuery,
};

mod constants;
mod models;

pub fn create_router() -> Router {
    Router::new()
        .route("/create-user", post(create_user))
        .route("/login", post(login))
        .route("/refresh-credentials", any(refresh_credentials))
        .route("/sealing-key", get(get_sealing_key))
}

async fn create_user(
    cookie_jar: CookieJar,
    Domain(domain): Domain,
    crypto_manager: Extension<CryptoManager>,
    Extension(db_manager): Extension<DbManager>,
    Extension(redis_manager): Extension<RedisManager>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(), Error> {
    let password_hash = crypto_manager.generate_hash(&payload.password_hash)?;
    let user_data = db_manager
        .create_user_from_invitation(&payload.invitation_id, &payload.username, &password_hash)
        .await?
        .ok_or(Error::bad_credentials())?;

    let sealing_key = Secret::default().get();

    generate_and_store_tokens(
        cookie_jar,
        domain,
        redis_manager,
        user_data.id,
        sealing_key,
        user_data.role,
        None,
    )
    .await?;

    Ok(())
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
        .ok_or(Error::bad_credentials())?;
    let valid_password =
        crypto_manager.matches(&payload.password_hash, &user_data.password_hash)?;

    if !valid_password {
        Err(Error::bad_credentials())?
    }

    let sealing_key = Secret::default().get();

    generate_and_store_tokens(
        cookie_jar,
        domain,
        redis_manager,
        user_data.id,
        sealing_key,
        user_data.role,
        None,
    )
    .await?;

    Ok(())
}

async fn refresh_credentials(
    cookie_jar: CookieJar,
    Domain(domain): Domain,
    URIScheme(uri_scheme): URIScheme,
    Extension(redis_manager): Extension<RedisManager>,
    payload: Query<RefreshCredentialsQuery>,
) -> Result<Response, Error> {
    let refresh_token = cookie_jar
        .get(REFRESH_TOKEN_COOKIE_NAME)
        .map(|cookie| cookie.value().to_owned());

    let refresh_token_item = if let Some(refresh_token) = refresh_token.clone() {
        redis_manager.get_refresh_token_item(&refresh_token).await
    } else {
        Ok(None)
    };

    let decoded_return_uri = decode(&payload.return_uri)
        .map_err(|_| Error::serialisation())?
        .to_string();
    if !decoded_return_uri.starts_with("https://") {
        return Err(Error::bad_return_uri());
    }
    let return_uri_domain: Option<String> = decoded_return_uri[8..]
        .split("/")
        .map(|part| part.to_string())
        .next();
    if return_uri_domain
        .filter(|return_uri_domain| return_uri_domain.ends_with(&domain))
        .is_none()
    {
        return Err(Error::bad_return_uri());
    }

    let response = match (refresh_token, refresh_token_item) {
        (
            _,
            Ok(Some(RedisRefreshToken {
                refresh_token,
                kind: RedisRefreshTokenKind::Active(data),
            })),
        ) => {
            generate_and_store_tokens(
                cookie_jar,
                domain,
                redis_manager,
                data.user_id,
                data.sealing_key,
                data.role,
                Some(refresh_token),
            )
            .await?;
            Redirect::temporary(&decoded_return_uri).into_response()
        }
        (
            _,
            Ok(Some(RedisRefreshToken {
                refresh_token: _,
                kind: RedisRefreshTokenKind::Refreshed(data),
            })),
        ) => {
            let new_refresh_token_item = redis_manager
                .get_refresh_token_item(&data.fresh_refresh_token)
                .await?;
            if new_refresh_token_item.is_some() {
                set_auth_cookies(
                    cookie_jar,
                    domain,
                    data.fresh_access_token,
                    data.fresh_refresh_token,
                );
                Redirect::temporary(&decoded_return_uri).into_response()
            } else {
                erase_cookies_and_redirect_to_login(
                    cookie_jar,
                    payload.return_uri.clone(),
                    domain,
                    uri_scheme,
                )
            }
        }
        (None, _) | (_, Ok(None)) => erase_cookies_and_redirect_to_login(
            cookie_jar,
            payload.return_uri.clone(),
            domain,
            uri_scheme,
        ),
        (_, Err(error)) => error.into_response(),
    };

    Ok(response)
}

async fn get_sealing_key(
    cookie_jar: CookieJar,
    Extension(redis_manager): Extension<RedisManager>,
) -> Result<Json<GetSealingKeyResponse>, Error> {
    let access_token = cookie_jar
        .get(ACCESS_TOKEN_COOKIE_NAME)
        .map(|cookie| cookie.value().to_owned());

    let access_token_item = if let Some(access_token) = access_token.clone() {
        redis_manager.get_access_token_item(&access_token).await
    } else {
        Ok(None)
    }?;

    if let Some(access_token_item) = access_token_item {
        Ok(Json(GetSealingKeyResponse {
            sealing_key: access_token_item.sealing_key,
        }))
    } else {
        Err(Error::bad_credentials())
    }
}

fn auth_cookie<'a>(name: String, value: String) -> Cookie<'a> {
    let mut cookie = Cookie::new(name, value);

    cookie.set_http_only(true);
    cookie.set_max_age(Some(CREDENTIALS_DURATION));
    cookie.set_same_site(SameSite::Strict);
    cookie.set_secure(true);

    cookie
}

async fn generate_and_store_tokens(
    cookie_jar: CookieJar,
    domain: String,
    redis_manager: RedisManager,
    user_id: u32,
    sealing_key: String,
    role: UserRole,
    old_refresh_token: Option<String>,
) -> Result<(), Error> {
    let access_token = Secret::default().get();
    let refresh_token = Secret::default().get();

    if let Some(old_refresh_token) = old_refresh_token {
        redis_manager
            .store_refreshed_auth_tokens(
                &old_refresh_token,
                &access_token,
                &refresh_token,
                user_id,
                &sealing_key,
                &role,
            )
            .await?;
    } else {
        redis_manager
            .store_active_auth_tokens(&access_token, &refresh_token, user_id, &sealing_key, &role)
            .await?;
    }

    set_auth_cookies(cookie_jar, domain, access_token, refresh_token);

    Ok(())
}

fn set_auth_cookies(
    cookie_jar: CookieJar,
    domain: String,
    access_token: String,
    refresh_token: String,
) {
    let mut access_token_cookie = auth_cookie(ACCESS_TOKEN_COOKIE_NAME.to_string(), access_token);
    access_token_cookie.set_domain(domain);

    let mut refresh_token_cookie =
        auth_cookie(REFRESH_TOKEN_COOKIE_NAME.to_string(), refresh_token);
    refresh_token_cookie.set_path("/api/refresh-credentials");

    let _ = cookie_jar
        .add(access_token_cookie)
        .add(refresh_token_cookie);
}

fn erase_cookies_and_redirect_to_login(
    cookie_jar: CookieJar,
    encoded_return_uri: String,
    domain: String,
    uri_scheme: String,
) -> Response {
    let mut access_token_cookie = auth_cookie(ACCESS_TOKEN_COOKIE_NAME.to_string(), "".to_string());
    access_token_cookie.set_max_age(Duration::ZERO);

    let mut refresh_token_cookie =
        auth_cookie(REFRESH_TOKEN_COOKIE_NAME.to_string(), "".to_string());
    refresh_token_cookie.set_max_age(Duration::ZERO);

    let _ = cookie_jar
        .add(access_token_cookie)
        .add(refresh_token_cookie);

    let redirect_uri_prefix = format!("{}auth.{}", uri_scheme, domain);

    let redirect_uri = format!(
        "{}/login?return_uri={}",
        redirect_uri_prefix, encoded_return_uri
    );
    Redirect::to(&redirect_uri).into_response()
}
