use axum::http::StatusCode;
use axum::http::header::ToStrError;
use axum::response::{IntoResponse, Response};
use std::fmt::{Display, Formatter};

use crate::managers::container::error::Error as ContainerError;
use crate::managers::crypto::error::Error as CryptoError;
use crate::managers::db::error::Error as DbError;
use crate::managers::redis::error::Error as RedisError;
use crate::managers::secrets::error::Error as SecretsError;

#[derive(Debug, Clone)]
pub enum Error {
    Container(ContainerError),
    Crypto(CryptoError),
    Db(DbError),
    Io(String),
    Redis(RedisError),
    Secrets(SecretsError),
    BadCredentials,
    BadReturnUri,
    StringConversion,
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:#?}", self)
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::Container(ref error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{:#?}: {:#?}", self, error),
            ),
            Self::Crypto(ref error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{:#?}: {:#?}", self, error),
            ),
            Self::Db(ref error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{:#?}: {:#?}", self, error),
            ),
            Self::Io(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            Self::Redis(ref error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{:#?}: {:#?}", self, error),
            ),
            Self::Secrets(ref error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{:#?}: {:#?}", self, error),
            ),
            Self::BadCredentials => (StatusCode::UNAUTHORIZED, "bad credentials".to_string()),
            Self::BadReturnUri => (StatusCode::UNAUTHORIZED, "bad return uri".to_string()),
            Self::StringConversion => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "string conversion failed".to_string(),
            ),
        }
        .into_response()
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        let message = format!("{:?}", value);
        Self::Io(message)
    }
}

impl From<ContainerError> for Error {
    fn from(value: ContainerError) -> Self {
        Self::Container(value)
    }
}

impl From<CryptoError> for Error {
    fn from(value: CryptoError) -> Self {
        Self::Crypto(value)
    }
}

impl From<DbError> for Error {
    fn from(value: DbError) -> Self {
        Self::Db(value)
    }
}

impl From<RedisError> for Error {
    fn from(value: RedisError) -> Self {
        Self::Redis(value)
    }
}

impl From<SecretsError> for Error {
    fn from(value: SecretsError) -> Self {
        Self::Secrets(value)
    }
}

impl From<ToStrError> for Error {
    fn from(_: ToStrError) -> Self {
        Self::StringConversion
    }
}
