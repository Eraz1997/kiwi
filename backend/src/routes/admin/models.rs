use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{
    managers::{container::models::Log, db::models::ServiceData},
    models::UserRole,
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub username: String,
    pub role: UserRole,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericAdminData {
    pub me: User,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersData {
    pub me: User,
    pub users: Vec<User>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateInfo {
    pub issuer: String,
    pub expiration_date: NaiveDateTime,
    pub new_pending_order: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificatesData {
    pub me: User,
    pub certificate: CertificateInfo,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicDnsData {
    pub me: User,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServicesData {
    pub me: User,
    pub services: Vec<ServiceData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailedServiceData {
    pub general_info: ServiceData,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditServiceData {
    pub me: User,
    pub service: DetailedServiceData,
    pub logs: Vec<Log>,
}
