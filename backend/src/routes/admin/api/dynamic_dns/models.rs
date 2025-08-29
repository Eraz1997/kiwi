use serde::{Deserialize, Serialize};

use crate::managers::secrets::models::DynamicDnsApiConfiguration;

#[derive(Serialize, Deserialize)]
pub struct GetDynamicDnsConfigurationResponse {
    pub enabled: bool,
}

pub type EnableDynamicDnsConfigurationRequest = DynamicDnsApiConfiguration;
