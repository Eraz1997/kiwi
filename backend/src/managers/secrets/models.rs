use rand::Rng;
use rand::distr::Alphanumeric;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Secret {
    value: String,
}

impl Secret {
    pub fn generate(length: usize) -> Self {
        let value = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();
        Self { value }
    }

    pub fn get(&self) -> String {
        self.value.clone()
    }
}

impl Default for Secret {
    fn default() -> Self {
        Self::generate(64)
    }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Secrets {
    pub crypto_pepper: Secret,
    pub db_admin_username: Secret,
    pub db_admin_password: Secret,
    pub redis_admin_password: Secret,
}
