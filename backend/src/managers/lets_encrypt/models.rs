use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCertificateOrder {
    pub order_url: String,
    pub dns_record_name: String,
    pub dns_record_value: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CertificateVerificationStatus {
    Pending,
    Success,
    Error,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateInfo {
    pub issuer: String,
    pub expiration_date: NaiveDateTime,
}
