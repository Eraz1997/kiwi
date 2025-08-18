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

    pub fn inconsistent_name() -> Self {
        Self {
            code: StatusCode::BAD_REQUEST,
            message: "service name cannot be changed".to_string(),
        }
    }

    pub fn inconsistent_port() -> Self {
        Self {
            code: StatusCode::BAD_REQUEST,
            message: "service port cannot be changed".to_string(),
        }
    }
}
