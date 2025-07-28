use serde::{Deserialize, Serialize};

use crate::models::UserRole;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub role: UserRole,
}
pub type GetUsersResponse = Vec<User>;
pub type GetMeResponse = User;

#[derive(Serialize, Deserialize)]
pub struct DeleteUserRequest {
    pub username: String,
}
