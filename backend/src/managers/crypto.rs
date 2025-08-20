use argon2::{
    Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::error::Error;

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

    pub fn generate_hash(&self, clear_text: &str) -> Result<String, Error> {
        let hasher = self.get_hasher()?;
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = hasher.hash_password(clear_text.as_bytes(), &salt)?;
        Ok(password_hash.to_string())
    }

    pub fn matches(&self, clear_text: &str, hashed_text: &str) -> Result<bool, Error> {
        let hasher = self.get_hasher()?;
        let password_hash = PasswordHash::new(hashed_text)?;
        let matches = hasher
            .verify_password(clear_text.as_bytes(), &password_hash)
            .is_ok();
        Ok(matches)
    }

    fn get_hasher(&self) -> Result<Argon2<'_>, Error> {
        Ok(Argon2::new_with_secret(
            self.pepper.as_bytes(),
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            Params::DEFAULT,
        )?)
    }
}
