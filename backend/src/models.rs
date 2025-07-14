use std::str::FromStr;

use postgres_types::{FromSql, ToSql};
use rand::Rng;
use rand::distr::Alphanumeric;
use serde::{Deserialize, Serialize};

use crate::error::Error;

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

#[derive(Clone, Debug, FromSql, ToSql)]
#[postgres(name = "user_role")]
pub enum UserRole {
    Admin,
    Customer,
}

impl UserRole {
    pub fn has_permissions(&self, target_role: &Self) -> bool {
        matches!(
            (self, target_role),
            (&Self::Admin, _) | (&Self::Customer, &Self::Customer)
        )
    }
}

impl FromStr for UserRole {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Admin" => Ok(Self::Admin),
            "Customer" => Ok(Self::Customer),
            _ => Err(Error::serialisation()),
        }
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Admin => write!(f, "Admin"),
            Self::Customer => write!(f, "Customer"),
        }
    }
}
