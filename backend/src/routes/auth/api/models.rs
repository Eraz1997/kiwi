use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshCredentialsQuery {
    pub return_uri: String,
}
