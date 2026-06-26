use crate::error::Error;
use axum::http::StatusCode;

impl Error {
    pub fn text_is_too_long() -> Self {
        Self {
            code: StatusCode::BAD_REQUEST,
            message: "text is too long".to_string(),
        }
    }
}
