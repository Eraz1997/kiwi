use std::{io, string::FromUtf8Error};

#[derive(Debug, Clone)]
pub enum Error {
    Io,
    HomeDirectoryNotFound,
    JsonConversion,
    PathConversion,
    StringConversion,
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Self {
        Self::Io
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Self::JsonConversion
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_: FromUtf8Error) -> Self {
        Self::StringConversion
    }
}
