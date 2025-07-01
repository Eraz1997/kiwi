#[derive(Debug, Clone)]
pub enum Error {
    Client,
}

impl From<reqwest::Error> for Error {
    fn from(_: reqwest::Error) -> Self {
        Self::Client
    }
}
