use std::fmt;

use rand::Rng;
use rand::distr::Alphanumeric;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
pub enum DynamicDnsProvider {
    GoDaddy,
}

impl fmt::Display for DynamicDnsProvider {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GoDaddy => write!(formatter, "GoDaddy"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DynamicDnsApiConfiguration {
    pub provider: DynamicDnsProvider,
    pub authorization_header: Secret,
    pub domain: Secret,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Secrets {
    pub crypto_pepper: Secret,
    pub db_admin_username: Secret,
    pub db_admin_password: Secret,
    pub redis_admin_password: Secret,
    pub dynamic_dns_api_configuration: Option<DynamicDnsApiConfiguration>,
}
