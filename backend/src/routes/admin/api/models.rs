use serde::{Deserialize, Serialize};

use crate::models::UserRole;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub role: UserRole,
}
pub type GetUsersResponse = Vec<User>;
