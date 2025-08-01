use crate::error::Error;
use axum::http::StatusCode;

impl Error {
    pub fn container_not_found() -> Self {
        Self {
            code: StatusCode::NOT_FOUND,
            message: "cannot find queried container".to_string(),
        }
    }

    pub fn port_in_use(port: &u16) -> Self {
        Self {
            code: StatusCode::BAD_REQUEST,
            message: format!("port {} in use", port),
        }
    }
}
