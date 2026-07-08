use axum::{
    extract::{Request, State},
    http::HeaderValue,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::CookieJar;
use urlencoding::encode;

use crate::{
    constants::{ACCESS_TOKEN_COOKIE_NAME, KIWI_USER_ID_HEADER_NAME, KIWI_USERNAME_HEADER_NAME},
    error::Error,
    extractors::{Domain, DomainAndSubdomain, FullOriginalUri},
    managers::redis::models::RedisServiceAuthorisation,
    models::UserRole,
    state::AppState,
};

pub async fn authentication_middleware(
    State(state): State<AppState>,
    cookie_jar: CookieJar,
    Domain(domain): Domain,
    DomainAndSubdomain(domain_and_subdomain): DomainAndSubdomain,
    FullOriginalUri(original_uri): FullOriginalUri,
    mut request: Request,
    next: Next,
) -> Response {
    // Remove any abused auth header
    request.headers_mut().remove(KIWI_USER_ID_HEADER_NAME);
    request.headers_mut().remove(KIWI_USERNAME_HEADER_NAME);

    let service = request
        .uri()
        .path()
        .split("/")
        .find(|part| !part.is_empty())
        .unwrap_or_default();

    let required_role = if service == "admin" {
        Some(UserRole::Admin)
    } else {
        match state.redis_manager.get_service_authorisation(service).await {
            Ok(Some(RedisServiceAuthorisation {
                service_name: _,
                required_role,
            })) => required_role,
            Ok(None) => match state.db_manager.get_service_data(service).await {
                Ok(Some(service_data)) => {
                    let required_role = service_data.container_configuration.required_role;
                    if let Err(error) = state
                        .redis_manager
                        .store_service_authorisation(service, required_role.clone())
                        .await
                    {
                        tracing::error!(
                            "failed to store service authorisation information on Redis: {}",
                            error
                        );
                    }
                    required_role
                }
                Ok(None) => None,
                Err(_) => return Error::internal_authorisation_failure().into_response(),
            },
            Err(_) => return Error::internal_authorisation_failure().into_response(),
        }
    };

    let access_token = cookie_jar
        .get(ACCESS_TOKEN_COOKIE_NAME)
        .map(|cookie| cookie.value().to_owned());

    let access_token_item = if let Some(access_token) = access_token.clone() {
        state
            .redis_manager
            .get_access_token_item(&access_token)
            .await
    } else {
        Ok(None)
    };

    let original_uri = original_uri.to_string();
    let encoded_original_uri = encode(&original_uri);

    match (required_role, access_token, access_token_item) {
        (required_role, Some(_), Ok(Some(access_token_item))) => {
            let user_id_string = access_token_item.user_id.to_string();

            if let Some(required_role) = required_role
                && !access_token_item.role.has_permissions(&required_role)
            {
                return Error::bad_permissions().into_response();
            }

            if let (Ok(user_id_header_value), Ok(username_header_value)) = (
                HeaderValue::from_str(&user_id_string),
                HeaderValue::from_str(&access_token_item.username),
            ) {
                request
                    .headers_mut()
                    .append(KIWI_USER_ID_HEADER_NAME, user_id_header_value);
                request
                    .headers_mut()
                    .append(KIWI_USERNAME_HEADER_NAME, username_header_value);
                next.run(request).await
            } else {
                Error::serialisation().into_response()
            }
        }
        (Some(_), None, Ok(_)) => {
            let redirect_uri = format!(
                "https://auth.{}/login?return_uri={}",
                domain, encoded_original_uri
            );
            Redirect::to(&redirect_uri).into_response()
        }
        (_, Some(_), Ok(None)) => {
            if service == "auth" {
                next.run(request).await
            } else {
                let redirect_uri = format!(
                    "https://{}/api/refresh-credentials?return_uri={}",
                    domain_and_subdomain, encoded_original_uri
                );
                Redirect::temporary(&redirect_uri).into_response()
            }
        }
        (None, None, Ok(_)) => next.run(request).await,
        (_, _, Err(error)) => error.into_response(),
    }
}
