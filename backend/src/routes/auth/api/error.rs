use crate::error::Error;
use axum::http::StatusCode;

impl Error {
    pub fn invalid_username() -> Self {
        Self {
            code: StatusCode::BAD_REQUEST,
            message: "invalid username".to_string(),
        }
    }

    pub fn invalid_password() -> Self {
        Self {
            code: StatusCode::BAD_REQUEST,
            message: "invalid password".to_string(),
        }
    }
}
