use crate::error::Error;
use axum::http::StatusCode;

impl Error {
    pub fn provider_test_failed() -> Self {
        Self {
            code: StatusCode::UNAUTHORIZED,
            message: "could not authenticate to the dynamic dns provider".to_string(),
        }
    }
}
