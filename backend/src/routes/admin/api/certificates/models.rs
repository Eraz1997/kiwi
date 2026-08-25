use serde::{Deserialize, Serialize};

use crate::routes::admin::models::CertificateInfo;

pub type GetCertificateInfoResponse = CertificateInfo;

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
