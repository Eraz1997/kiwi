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
            .take(64)
            .map(char::from)
            .collect();
        Self { value }
    }
}
