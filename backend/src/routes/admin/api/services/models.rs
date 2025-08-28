use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::managers::{container::models::Log, db::models::ServiceData};

#[derive(Serialize, Deserialize)]
pub struct GetServicesResponse {
    pub services: Vec<ServiceData>,
}

#[derive(Serialize, Deserialize)]
pub struct GetServiceResponse {
    pub general_info: ServiceData,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetLogsQuery {
    pub from_date: NaiveDateTime,
    pub to_date: NaiveDateTime,
}

pub type GetLogsResponse = Vec<Log>;
