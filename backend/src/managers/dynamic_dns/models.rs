use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DnsRecordValue {
    pub data: String,
}
