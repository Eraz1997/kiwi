use axum::extract::Query;
use axum::response::Redirect;
use axum::routing::{any, get, post};
use axum::{Extension, Json, Router};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};
use models::LoginRequest;
use regex::Regex;
use time::Duration;
use urlencoding::decode;
use zxcvbn::{Score, zxcvbn};

use crate::constants::{
    ACCESS_TOKEN_COOKIE_NAME, LOCALHOST_DOMAIN_WITH_COLON, LOGOUT_REFRESH_TOKEN_COPY_NAME,
    REFRESH_TOKEN_COOKIE_NAME,
};
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
mod error;
mod models;

pub fn create_router() -> Router {
    Router::new()
        .route("/create-user", post(create_user))
        .route("/login", post(login))
        .route("/logout", post(logout))
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
) -> Result<CookieJar, Error> {
    let username_regex = Regex::new(r"^[a-zA-Z0-9.-_]{6,32}$")?;
    if !username_regex.is_match(&payload.username) {
        return Err(Error::invalid_username());
    }

    let password_score = zxcvbn(&payload.password_hash, &[&payload.username]);
    if password_score.score() != Score::Four {
        return Err(Error::invalid_password());
    }

    let password_hash = crypto_manager.generate_hash(&payload.password_hash)?;
    let user_data = db_manager
        .create_user_from_invitation(&payload.invitation_id, &payload.username, &password_hash)
        .await?
        .ok_or(Error::bad_credentials())?;

    let sealing_key = Secret::generate(32 + 16).get(); // AES-CBC key + iv

    generate_and_store_tokens(
        cookie_jar,
        domain,
        redis_manager,
        user_data.id,
        sealing_key,
        user_data.role,
        None,
    )
    .await
}

async fn login(
    cookie_jar: CookieJar,
    Domain(domain): Domain,
    crypto_manager: Extension<CryptoManager>,
    Extension(db_manager): Extension<DbManager>,
    Extension(redis_manager): Extension<RedisManager>,
    Json(payload): Json<LoginRequest>,
) -> Result<CookieJar, Error> {
    let user_data = db_manager
        .get_user_data(&payload.username)
        .await?
        .ok_or(Error::bad_credentials())?;
    let valid_password =
        crypto_manager.matches(&payload.password_hash, &user_data.password_hash)?;

    if !valid_password {
        Err(Error::bad_credentials())?
    }

    let sealing_key = Secret::generate(32 + 16).get(); // AES-CBC key + iv

    generate_and_store_tokens(
        cookie_jar,
        domain,
        redis_manager,
        user_data.id,
        sealing_key,
        user_data.role,
        None,
    )
    .await
}

async fn logout(
    cookie_jar: CookieJar,
    Domain(domain): Domain,
    URIScheme(uri_scheme): URIScheme,
    Extension(redis_manager): Extension<RedisManager>,
) -> Result<CookieJar, Error> {
    let refresh_token = cookie_jar
        .get(LOGOUT_REFRESH_TOKEN_COPY_NAME)
        .map(|cookie| cookie.value().to_owned());

    match refresh_token {
        Some(refresh_token) => {
            redis_manager.erase_refresh_token(&refresh_token).await?;
            let (cookie_jar, _) =
                erase_cookies_and_redirect_to_login(cookie_jar, None, domain, uri_scheme);
            Ok(cookie_jar)
        }
        None => Ok(cookie_jar),
    }
}

async fn refresh_credentials(
    cookie_jar: CookieJar,
    Domain(domain): Domain,
    URIScheme(uri_scheme): URIScheme,
    Extension(redis_manager): Extension<RedisManager>,
    payload: Query<RefreshCredentialsQuery>,
) -> Result<(CookieJar, Redirect), Error> {
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
    let return_uri_domain = decoded_return_uri
        .strip_prefix(&uri_scheme)
        .and_then(|uri_domain| uri_domain.split("/").next())
        .ok_or(Error::bad_return_uri())?
        .to_string();
    if !return_uri_domain.ends_with(&domain) {
        return Err(Error::bad_return_uri());
    }

    match (refresh_token, refresh_token_item) {
        (
            _,
            Ok(Some(RedisRefreshToken {
                refresh_token,
                kind: RedisRefreshTokenKind::Active(data),
            })),
        ) => {
            let cookie_jar = generate_and_store_tokens(
                cookie_jar,
                domain,
                redis_manager,
                data.user_id,
                data.sealing_key,
                data.role,
                Some(refresh_token),
            )
            .await?;
            Ok((cookie_jar, Redirect::temporary(&decoded_return_uri)))
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
                let cookie_jar = set_auth_cookies(
                    cookie_jar,
                    domain,
                    data.fresh_access_token,
                    data.fresh_refresh_token,
                );
                Ok((cookie_jar, Redirect::temporary(&decoded_return_uri)))
            } else {
                Ok(erase_cookies_and_redirect_to_login(
                    cookie_jar,
                    Some(payload.return_uri.clone()),
                    domain,
                    uri_scheme,
                ))
            }
        }
        (None, _) | (_, Ok(None)) => Ok(erase_cookies_and_redirect_to_login(
            cookie_jar,
            Some(payload.return_uri.clone()),
            domain,
            uri_scheme,
        )),
        (_, Err(error)) => Err(error),
    }
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
        let key = &access_token_item.sealing_key[..32];
        let iv = &access_token_item.sealing_key[32..];
        Ok(Json(GetSealingKeyResponse {
            key: key.to_string(),
            iv: iv.to_string(),
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
    user_id: i64,
    sealing_key: String,
    role: UserRole,
    old_refresh_token: Option<String>,
) -> Result<CookieJar, Error> {
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

    let cookie_jar = set_auth_cookies(cookie_jar, domain, access_token, refresh_token);
    Ok(cookie_jar)
}

fn set_auth_cookies(
    cookie_jar: CookieJar,
    domain: String,
    access_token: String,
    refresh_token: String,
) -> CookieJar {
    let domain_parts: Vec<String> = domain.split(":").map(|value| value.to_string()).collect();
    let domain_without_port = format!(".{}", domain_parts[0]);

    let mut access_token_cookie = auth_cookie(ACCESS_TOKEN_COOKIE_NAME.to_string(), access_token);
    access_token_cookie.set_domain(domain_without_port);
    access_token_cookie.set_path("/");

    let mut refresh_token_cookie =
        auth_cookie(REFRESH_TOKEN_COOKIE_NAME.to_string(), refresh_token.clone());
    refresh_token_cookie.set_path("/api/refresh-credentials");

    let mut logout_refresh_token_cookie =
        auth_cookie(LOGOUT_REFRESH_TOKEN_COPY_NAME.to_string(), refresh_token);
    logout_refresh_token_cookie.set_path("/api/logout");

    if domain.starts_with(LOCALHOST_DOMAIN_WITH_COLON) {
        access_token_cookie.set_secure(false);
        refresh_token_cookie.set_secure(false);
        logout_refresh_token_cookie.set_secure(false);
    }

    cookie_jar
        .add(access_token_cookie)
        .add(refresh_token_cookie)
        .add(logout_refresh_token_cookie)
}

fn erase_cookies_and_redirect_to_login(
    cookie_jar: CookieJar,
    encoded_return_uri: Option<String>,
    domain: String,
    uri_scheme: String,
) -> (CookieJar, Redirect) {
    let domain_parts: Vec<String> = domain.split(":").map(|value| value.to_string()).collect();
    let domain_without_port = format!(".{}", domain_parts[0]);

    let mut access_token_cookie = auth_cookie(ACCESS_TOKEN_COOKIE_NAME.to_string(), "".to_string());
    access_token_cookie.set_domain(domain_without_port);
    access_token_cookie.set_path("/");
    access_token_cookie.set_max_age(Duration::ZERO);

    let mut refresh_token_cookie =
        auth_cookie(REFRESH_TOKEN_COOKIE_NAME.to_string(), "".to_string());
    refresh_token_cookie.set_path("/api/refresh-credentials");
    refresh_token_cookie.set_max_age(Duration::ZERO);

    let mut logout_refresh_token_cookie =
        auth_cookie(LOGOUT_REFRESH_TOKEN_COPY_NAME.to_string(), "".to_string());
    logout_refresh_token_cookie.set_path("/api/logout");
    logout_refresh_token_cookie.set_max_age(Duration::ZERO);

    if domain.starts_with(LOCALHOST_DOMAIN_WITH_COLON) {
        access_token_cookie.set_secure(false);
        refresh_token_cookie.set_secure(false);
        logout_refresh_token_cookie.set_secure(false);
    }

    let cookie_jar = cookie_jar
        .add(access_token_cookie)
        .add(refresh_token_cookie)
        .add(logout_refresh_token_cookie);

    let redirect_uri_prefix = format!("{}auth.{}", uri_scheme, domain);

    let redirect_uri = if let Some(encoded_return_uri) = encoded_return_uri {
        format!(
            "{}/login?return_uri={}",
            redirect_uri_prefix, encoded_return_uri
        )
    } else {
        format!("{}/login", redirect_uri_prefix)
    };
    (cookie_jar, Redirect::to(&redirect_uri))
}
