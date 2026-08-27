use std::str::FromStr;

use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Clone, Debug, FromSql, ToSql, Serialize, Deserialize)]
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

pub enum ServerAction {
    RestartWithoutDependenciesInit,
    CloseDueToUnexpectedError,
}
