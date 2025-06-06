use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::fmt::{Display, Formatter};

use crate::managers::container::error::Error as ContainerError;
use crate::managers::db::error::Error as DbError;
use crate::managers::secrets::error::Error as SecretsError;

#[derive(Debug, Clone)]
pub enum Error {
    Io(String),
    Container(ContainerError),
    Db(DbError),
    Secrets(SecretsError),
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
            Error::Io(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            Error::Container(ref error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{:#?}: {:#?}", self, error),
            ),
            Error::Db(ref error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{:#?}: {:#?}", self, error),
            ),
            Error::Secrets(ref error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{:#?}: {:#?}", self, error),
            ),
        }
        .into_response()
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        let message = format!("{:?}", value);
        Error::Io(message)
    }
}

impl From<ContainerError> for Error {
    fn from(value: ContainerError) -> Self {
        Error::Container(value)
    }
}

impl From<DbError> for Error {
    fn from(value: DbError) -> Self {
        Error::Db(value)
    }
}

impl From<SecretsError> for Error {
    fn from(value: SecretsError) -> Self {
        Error::Secrets(value)
    }
}
