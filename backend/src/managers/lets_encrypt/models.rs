use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewCertificateOrder {
    pub order_url: String,
    pub dns_record_name: String,
    pub dns_record_value: String,
}

#[derive(Serialize, Deserialize)]
pub enum CertificateVerificationStatus {
    Pending,
    Success,
    Error,
}

#[derive(Serialize, Deserialize)]
pub struct CertificateInfo {
    pub issuer: String,
    pub expiration_date: NaiveDateTime,
}
