use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Serialize, Deserialize)]
pub struct CreateUserInvitationRequest {
    pub role: UserRole,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserInvitationResponse {
    pub invitation_id: Uuid,
}
