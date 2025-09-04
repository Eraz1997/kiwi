use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use deadpool_postgres::{CreatePoolError, PoolError};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Error {
    pub code: StatusCode,
    pub message: String,
}

impl Error {
    pub fn bad_credentials() -> Self {
        Self {
            code: StatusCode::UNAUTHORIZED,
            message: "bad credentials".to_string(),
        }
    }

    pub fn bad_return_uri() -> Self {
        Self {
            code: StatusCode::UNAUTHORIZED,
            message: "bad return uri".to_string(),
        }
    }

    pub fn bad_permissions() -> Self {
        Self {
            code: StatusCode::FORBIDDEN,
            message: "you don't have enough permissions to access this service".to_string(),
        }
    }

    pub fn serialisation() -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "serialisation failed".to_string(),
        }
    }

    pub fn unauthorised() -> Self {
        Self {
            code: StatusCode::UNAUTHORIZED,
            message: "something went wrong with your credentials".to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Error {}: {}", self.code, self.message)
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let message = if self.code == StatusCode::INTERNAL_SERVER_ERROR {
            tracing::error!("{}", self);
            "internal server error".to_string()
        } else {
            self.message
        };
        (self.code, message).into_response()
    }
}

impl From<CreatePoolError> for Error {
    fn from(error: CreatePoolError) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string(),
        }
    }
}

impl From<PoolError> for Error {
    fn from(error: PoolError) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string(),
        }
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(error: tokio_postgres::Error) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string(),
        }
    }
}

impl From<refinery::Error> for Error {
    fn from(error: refinery::Error) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string(),
        }
    }
}

impl From<argon2::Error> for Error {
    fn from(error: argon2::Error) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string(),
        }
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(error: argon2::password_hash::Error) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string(),
        }
    }
}

impl From<fred::error::Error> for Error {
    fn from(error: fred::error::Error) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string(),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string(),
        }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string(),
        }
    }
}

impl From<bollard::errors::Error> for Error {
    fn from(error: bollard::errors::Error) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string(),
        }
    }
}

impl From<regex::Error> for Error {
    fn from(error: regex::Error) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string(),
        }
    }
}

impl From<instant_acme::Error> for Error {
    fn from(error: instant_acme::Error) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string(),
        }
    }
}

impl From<rcgen::Error> for Error {
    fn from(error: rcgen::Error) -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: error.to_string(),
        }
    }
}
