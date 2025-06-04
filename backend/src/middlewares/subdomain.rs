use axum::{
    extract::Request,
    http::{Uri, header::HOST},
};

pub fn subdomain_middleware(mut request: Request) -> Request {
    let original_headers = request.headers().clone();
    let subdomain = original_headers.get(HOST).and_then(|host| {
        let domains: Vec<&str> = host.to_str().unwrap_or_default().split(".").collect();
        if domains.len() != 3 {
            None
        } else {
            Some(domains[0])
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
