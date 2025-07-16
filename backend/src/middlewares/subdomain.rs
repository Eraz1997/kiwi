use axum::{
    extract::Request,
    http::{Uri, header::HOST},
};

use crate::constants::LOCALHOST_DOMAIN_WITH_COLON;

pub fn subdomain_middleware(mut request: Request) -> Request {
    let subdomain = request
        .uri()
        .authority()
        .map(|host| host.to_string())
        .or(request
            .headers()
            .get(HOST)
            .cloned()
            .and_then(|host_header| {
                host_header
                    .to_str()
                    .ok()
                    .map(|header_value| header_value.to_string())
            }))
        .and_then(|host| {
            let host_value = host.to_string();
            let domains: Vec<&str> = host_value.split(".").collect();
            if domains.len() == 3
                || (domains.len() == 2 && domains[1].starts_with(LOCALHOST_DOMAIN_WITH_COLON))
            {
                Some(domains[0].to_string())
            } else {
                None
            }
        });

    let uri_parts = request.uri().clone().into_parts();

    let requested_path_and_query = if let Some(path_and_query) = request.uri().path_and_query() {
        path_and_query.as_str()
    } else {
        ""
    };

    let path_and_query = match subdomain {
        Some(route) => format!("/{}{}", route, requested_path_and_query),
        None => "".to_string(),
    };

    let mut uri_builder = Uri::builder().path_and_query(path_and_query);

    if let Some(scheme) = uri_parts.scheme {
        uri_builder = uri_builder.scheme(scheme);
    }
    if let Some(authority) = uri_parts.authority {
        uri_builder = uri_builder.authority(authority);
    }

    *request.uri_mut() = uri_builder.build().unwrap_or_default();
    request
}
