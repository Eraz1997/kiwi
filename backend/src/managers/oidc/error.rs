use crate::error::Error;
use axum::http::StatusCode;

impl Error {
    pub fn invalid_header() -> Self {
        Self {
            code: StatusCode::BAD_REQUEST,
            message: "invalid token header".to_string(),
        }
    }

    pub fn invalid_key_id() -> Self {
        Self {
            code: StatusCode::BAD_REQUEST,
            message: "invalid key id (kid)".to_string(),
        }
    }

    pub fn invalid_key() -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "invalid jwk".to_string(),
        }
    }

    pub fn invalid_jwt() -> Self {
        Self {
            code: StatusCode::BAD_REQUEST,
            message: "invalid jwt".to_string(),
        }
    }
}
