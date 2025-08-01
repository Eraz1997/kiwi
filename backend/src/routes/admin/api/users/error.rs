use crate::error::Error;
use axum::http::StatusCode;

impl Error {
    pub fn cannot_delete_active_user() -> Self {
        Self {
            code: StatusCode::BAD_REQUEST,
            message: "cannot delete active user".to_string(),
        }
    }
}
