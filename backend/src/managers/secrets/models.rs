use serde::{Deserialize, Serialize};

use crate::models::Secret;

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Secrets {
    pub crypto_pepper: Secret,
    pub db_admin_username: Secret,
    pub db_admin_password: Secret,
    pub redis_admin_password: Secret,
}
