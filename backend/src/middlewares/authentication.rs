use axum::{
    Extension,
    extract::{OriginalUri, Request},
    http::HeaderValue,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::CookieJar;
use urlencoding::encode;

use crate::{
    constants::{ACCESS_TOKEN_COOKIE_NAME, KIWI_USER_ID_HEADER_NAME},
    error::Error,
    extractors::Domain,
    managers::redis::RedisManager,
};

pub async fn authentication_middleware(
    redis_manager: Extension<RedisManager>,
    cookie_jar: CookieJar,
    Domain(domain): Domain,
    original_uri: OriginalUri,
    mut request: Request,
    next: Next,
) -> Response {
    let service = request
        .uri()
        .path()
        .split("/")
        .find(|part| !part.is_empty())
        .unwrap_or_default();

    let is_authentication_required = service == "admin";

    let access_token = cookie_jar
        .get(ACCESS_TOKEN_COOKIE_NAME)
        .map(|cookie| cookie.value().to_owned());

    let user_id = if let Some(access_token) = access_token.clone() {
        redis_manager.get_access_token_user_id(&access_token).await
    } else {
        Ok(None)
    };

    let original_uri = original_uri.to_string();
    let encoded_original_uri = encode(&original_uri);
    let redirect_uri_prefix = match domain {
        Some(domain) => format!("https://auth.{}", domain),
        None => "/auth".to_string(),
    };

    match (is_authentication_required, access_token, user_id) {
        (true, Some(_), Ok(Some(user_id))) => {
            let user_id_string = user_id.to_string();
            if let Ok(user_id_header_value) = HeaderValue::from_str(&user_id_string) {
                request
                    .headers_mut()
                    .append(KIWI_USER_ID_HEADER_NAME, user_id_header_value);
                next.run(request).await
            } else {
                Error::StringConversion.into_response()
            }
        }
        (true, None, Ok(_)) => {
            let redirect_uri = format!(
                "{}/login?return_uri={}",
                redirect_uri_prefix, encoded_original_uri
            );
            Redirect::to(&redirect_uri).into_response()
        }
        (true, Some(_), Ok(None)) => {
            let redirect_uri = format!(
                "{}/api/refresh-credentials?return_uri={}",
                redirect_uri_prefix, encoded_original_uri
            );
            Redirect::temporary(&redirect_uri).into_response()
        }
        (false, _, Ok(_)) => next.run(request).await,
        (_, _, Err(error)) => Error::from(error).into_response(),
    }
}
