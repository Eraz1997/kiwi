use axum::{
    Extension,
    extract::Request,
    http::HeaderValue,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::CookieJar;
use urlencoding::encode;

use crate::{
    constants::{ACCESS_TOKEN_COOKIE_NAME, KIWI_USER_ID_HEADER_NAME},
    error::Error,
    extractors::{Domain, FullOriginalUri},
    managers::redis::RedisManager,
    models::UserRole,
};

pub async fn authentication_middleware(
    redis_manager: Extension<RedisManager>,
    cookie_jar: CookieJar,
    Domain(domain): Domain,
    FullOriginalUri(original_uri): FullOriginalUri,
    mut request: Request,
    next: Next,
) -> Response {
    // Remove any abused auth header
    request.headers_mut().remove(KIWI_USER_ID_HEADER_NAME);

    let service = request
        .uri()
        .path()
        .split("/")
        .find(|part| !part.is_empty())
        .unwrap_or_default();

    let required_role = match service {
        "admin" => Some(UserRole::Admin),
        _ => None,
    };

    let access_token = cookie_jar
        .get(ACCESS_TOKEN_COOKIE_NAME)
        .map(|cookie| cookie.value().to_owned());

    let access_token_item = if let Some(access_token) = access_token.clone() {
        redis_manager.get_access_token_item(&access_token).await
    } else {
        Ok(None)
    };

    let original_uri = original_uri.to_string();
    let encoded_original_uri = encode(&original_uri);
    let redirect_uri_prefix = format!("https://auth.{}", domain);

    match (required_role, access_token, access_token_item) {
        (Some(required_role), Some(_), Ok(Some(access_token_item))) => {
            let user_id_string = access_token_item.user_id.to_string();

            if !access_token_item.role.has_permissions(&required_role) {
                Error::bad_permissions().into_response()
            } else if let Ok(user_id_header_value) = HeaderValue::from_str(&user_id_string) {
                request
                    .headers_mut()
                    .append(KIWI_USER_ID_HEADER_NAME, user_id_header_value);
                next.run(request).await
            } else {
                Error::serialisation().into_response()
            }
        }
        (Some(_), None, Ok(_)) => {
            let redirect_uri = format!(
                "{}/login?return_uri={}",
                redirect_uri_prefix, encoded_original_uri
            );
            Redirect::to(&redirect_uri).into_response()
        }
        (Some(_), Some(_), Ok(None)) => {
            let redirect_uri = format!(
                "{}/api/refresh-credentials?return_uri={}",
                redirect_uri_prefix, encoded_original_uri
            );
            Redirect::temporary(&redirect_uri).into_response()
        }
        (None, _, Ok(_)) => next.run(request).await,
        (_, _, Err(error)) => error.into_response(),
    }
}
