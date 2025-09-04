use crate::error::Error;
use axum::http::StatusCode;

impl Error {
    pub fn order_not_found() -> Self {
        Self {
            code: StatusCode::NOT_FOUND,
            message: "certificate order not found".to_string(),
        }
    }
}
