use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeployServiceRequest {
    pub oidc_token: String,
    pub image_sha: String,
}
