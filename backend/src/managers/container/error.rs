#[derive(Debug, Clone)]
pub enum Error {
    ContainerIdNotFound,
    Docker,
    InvalidImageSha,
    RegexCreation,
}

impl From<bollard::errors::Error> for Error {
    fn from(_: bollard::errors::Error) -> Self {
        Self::Docker
    }
}

impl From<regex::Error> for Error {
    fn from(_: regex::Error) -> Self {
        Self::RegexCreation
    }
}
