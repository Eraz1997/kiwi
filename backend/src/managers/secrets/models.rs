use rand::Rng;
use rand::distr::Alphanumeric;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Secret {
    value: String,
}

impl Secret {
    pub fn get(&self) -> String {
        self.value.clone()
    }
}

impl Default for Secret {
    fn default() -> Self {
        let value = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();
        Self { value }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Secrets {
    pub db_admin_username: Secret,
    pub db_admin_password: Secret,
}
