use crate::error::Error;
use axum::http::StatusCode;

impl Error {
    pub fn container_id_not_found() -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "container id not found".to_string(),
        }
    }

    pub fn container_invalid_image_sha() -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "container image sha is not valid".to_string(),
        }
    }

    pub fn network_name_not_found() -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "network name not found".to_string(),
        }
    }
}
