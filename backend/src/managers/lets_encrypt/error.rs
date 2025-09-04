use crate::error::Error;
use axum::http::StatusCode;

impl Error {
    pub fn bad_order_status() -> Self {
        Self {
            code: StatusCode::EXPECTATION_FAILED,
            message: "the certificate order is not in expected pending status".to_string(),
        }
    }

    pub fn cannot_find_authorisation() -> Self {
        Self {
            code: StatusCode::EXPECTATION_FAILED,
            message: "order authorisation cannot be found".to_string(),
        }
    }

    pub fn bad_authorisation_status() -> Self {
        Self {
            code: StatusCode::EXPECTATION_FAILED,
            message: "order authorisation is in an unexpected status".to_string(),
        }
    }
}
