use crate::error::Error;
use axum::http::StatusCode;

impl Error {
    pub fn invalid_branch() -> Self {
        Self {
            code: StatusCode::FORBIDDEN,
            message: "only 'main' is accepted as valid deployment branch".to_string(),
        }
    }

    pub fn invalid_repo_for_service() -> Self {
        Self {
            code: StatusCode::FORBIDDEN,
            message: "service not found or not bound to this repo".to_string(),
        }
    }
}
