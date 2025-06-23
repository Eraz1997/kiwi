#[derive(Clone, Debug)]
pub enum Error {
    Redis,
    Serialisation,
}

impl From<fred::error::Error> for Error {
    fn from(_: fred::error::Error) -> Self {
        Self::Redis
    }
}
