use argon2::{Argon2, Params, PasswordHash, PasswordVerifier};

use crate::managers::crypto::error::Error;

pub mod error;

#[derive(Clone)]
pub struct CryptoManager {
    pepper: String,
}

impl CryptoManager {
    pub fn new(pepper: &str) -> Result<Self, Error> {
        tracing::info!("crypto manager initialised");
        Ok(Self {
            pepper: pepper.to_string(),
        })
    }

    pub fn matches(&self, clear_text: &str, hashed_text: &str) -> Result<bool, Error> {
        let hasher = self.get_hasher()?;
        let password_hash = PasswordHash::new(hashed_text)?;
        let matches = hasher
            .verify_password(clear_text.as_bytes(), &password_hash)
            .is_ok();
        Ok(matches)
    }

    fn get_hasher(&self) -> Result<Argon2, Error> {
        Ok(Argon2::new_with_secret(
            self.pepper.as_bytes(),
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            Params::DEFAULT,
        )?)
    }
}
