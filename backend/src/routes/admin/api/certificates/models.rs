use serde::{Deserialize, Serialize};

use crate::managers::lets_encrypt::models::CertificateInfo;

pub type GetCertificateInfoResponse = CertificateInfo;

#[derive(Serialize, Deserialize)]
pub struct OrderCertificateRequest {
    pub domain: String,
}

#[derive(Serialize, Deserialize)]
pub struct OrderCertificateResponse {
    pub dns_record_name: String,
    pub dns_record_value: String,
}
