#[derive(Clone, Debug)]
pub enum Error {
    Argon2,
    PasswordHash,
}

impl From<argon2::Error> for Error {
    fn from(_: argon2::Error) -> Self {
        Self::Argon2
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(_: argon2::password_hash::Error) -> Self {
        Self::PasswordHash
    }
}
