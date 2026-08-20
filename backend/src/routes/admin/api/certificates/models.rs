use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCertificateInfoResponse {
    pub issuer: String,
    pub expiration_date: NaiveDateTime,
    pub new_pending_order: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderCertificateRequest {
    pub domain: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderCertificateResponse {
    pub dns_record_name: String,
    pub dns_record_value: String,
}
