use serde::{Deserialize, Serialize};

use crate::managers::secrets::models::DynamicDnsApiConfiguration;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDynamicDnsConfigurationResponse {
    pub enabled: bool,
}

pub type EnableDynamicDnsConfigurationRequest = DynamicDnsApiConfiguration;
