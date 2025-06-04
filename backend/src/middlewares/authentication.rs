use axum::{extract::Request, middleware::Next, response::Response};

pub async fn authentication_middleware(request: Request, next: Next) -> Response {
    let service = request
        .uri()
        .path()
        .split("/")
        .find(|part| !part.is_empty())
        .unwrap_or_default();

    if service == "admin" {
        todo!("authentication not implemented")
    }

    next.run(request).await
}
