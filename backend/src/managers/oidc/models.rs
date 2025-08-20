use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Jwk {
    pub kty: String,
    pub use_: Option<String>,
    pub kid: String,
    pub alg: Option<String>,
    pub n: String,
    pub e: String,
    pub x5c: Option<Vec<String>>,
    pub x5t: Option<String>,
    pub x5t_s256: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GithubJwksResponse {
    pub keys: Vec<Jwk>,
}

#[derive(Serialize, Deserialize)]
pub struct GithubClaims {
    pub repository: String,
    #[serde(rename = "ref")]
    pub reference: String,
}
