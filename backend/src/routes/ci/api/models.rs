use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DeployServiceRequest {
    pub oidc_token: String,
    pub image_sha: String,
}
