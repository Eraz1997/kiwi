#[derive(Clone, Debug)]
pub enum Error {
    Redis,
}

impl From<fred::error::Error> for Error {
    fn from(_: fred::error::Error) -> Self {
        Self::Redis
    }
}
