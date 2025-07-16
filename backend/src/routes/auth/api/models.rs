use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshCredentialsQuery {
    pub return_uri: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetSealingKeyResponse {
    pub sealing_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password_hash: String,
    pub invitation_id: Uuid,
}
