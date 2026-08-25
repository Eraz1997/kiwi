use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{
    managers::{container::models::Log, db::models::ServiceData},
    routes::admin::models::DetailedServiceData,
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetServicesResponse {
    pub services: Vec<ServiceData>,
}

pub type GetServiceResponse = DetailedServiceData;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLogsQuery {
    pub from_date: NaiveDateTime,
    pub to_date: NaiveDateTime,
}

pub type GetLogsResponse = Vec<Log>;
