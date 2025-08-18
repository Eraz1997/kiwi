use crate::error::Error;
use axum::http::StatusCode;

impl Error {
    pub fn service_not_found() -> Self {
        Self {
            code: StatusCode::NOT_FOUND,
            message: "cannot find service".to_string(),
        }
    }
}
