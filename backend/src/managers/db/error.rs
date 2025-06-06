use deadpool_postgres::{CreatePoolError, PoolError};

#[derive(Debug, Clone)]
pub enum Error {
    ConnectionPoolCreation,
    ConnectionTest,
    Migrations,
    Pool,
    TokioPostgres,
}

impl From<CreatePoolError> for Error {
    fn from(_: CreatePoolError) -> Self {
        Self::ConnectionPoolCreation
    }
}

impl From<PoolError> for Error {
    fn from(_: PoolError) -> Self {
        Self::Pool
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(_: tokio_postgres::Error) -> Self {
        Self::TokioPostgres
    }
}

impl From<refinery::Error> for Error {
    fn from(_: refinery::Error) -> Self {
        Self::Migrations
    }
}
