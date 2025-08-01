use bollard::container::LogOutput;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha256::digest;

use crate::error::Error;
use crate::managers::db::constants::DATABASE_NAME;

#[derive(Clone, Serialize, Deserialize)]
pub struct ImageSha {
    value: String,
}

impl ImageSha {
    pub fn new(value: String) -> Result<Self, Error> {
        let re = Regex::new(r"^[0-9a-f]{64}$")?;
        if !re.is_match(&value) {
            Err(Error::container_invalid_image_sha())
        } else {
            Ok(Self { value })
        }
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ExposedPort {
    pub internal: u16,
    pub external: u16,
}

impl ExposedPort {
    pub fn symmetric(port: u16) -> Self {
        Self {
            internal: port,
            external: port,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EnvironmentVariable {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ContainerConfiguration {
    pub name: String,
    pub image_name: String,
    pub image_sha: ImageSha,
    pub exposed_ports: Vec<ExposedPort>,
    pub environment_variables: Vec<EnvironmentVariable>,
    pub secrets: Vec<EnvironmentVariable>,
    pub internal_secrets: Vec<EnvironmentVariable>,
    pub stateful_volume_paths: Vec<String>,
}

impl ContainerConfiguration {
    pub fn get_postgres_configuration(
        admin_username: &str,
        admin_password: &str,
    ) -> Result<Self, Error> {
        Ok(Self {
            name: "kiwi-postgres".to_string(),
            image_name: "postgres".to_string(),
            image_sha: ImageSha::new(
                "bcb90dc18910057ff49ce2ea157d8a0d534964090d39af959df41083f18c3318".to_string(),
            )?, // 17.5-alpine3.22
            exposed_ports: vec![ExposedPort::symmetric(5432)],
            environment_variables: vec![EnvironmentVariable {
                name: "POSTGRES_DB".to_string(),
                value: DATABASE_NAME.to_string(),
            }],
            secrets: vec![],
            internal_secrets: vec![
                EnvironmentVariable {
                    name: "POSTGRES_USER".to_string(),
                    value: admin_username.to_string(),
                },
                EnvironmentVariable {
                    name: "POSTGRES_PASSWORD".to_string(),
                    value: admin_password.to_string(),
                },
            ],
            stateful_volume_paths: vec!["/var/lib/postgresql/data".to_string()],
        })
    }

    pub fn get_redis_configuration(admin_password: &str) -> Result<Self, Error> {
        Ok(Self {
            name: "kiwi-redis".to_string(),
            image_name: "bitnami/redis".to_string(),
            image_sha: ImageSha::new(
                "d0f84da5011d75e3cda5516646ceb4ce6fa1eac50014c7090472af1f5ae80c91".to_string(),
            )?, // 8.0.2
            exposed_ports: vec![ExposedPort::symmetric(6379)],
            environment_variables: vec![],
            secrets: vec![],
            internal_secrets: vec![EnvironmentVariable {
                name: "REDIS_PASSWORD".to_string(),
                value: admin_password.to_string(),
            }],
            stateful_volume_paths: vec!["/bitnami/redis/data".to_string()],
        })
    }

    pub fn get_stateful_volume_id(self, path: &String) -> String {
        let raw_id = format!("{}-{}", self.name, path);
        let hashed_id = digest(raw_id);
        format!("{}-{}", self.name, hashed_id)
    }
}

#[derive(Serialize, Deserialize)]
pub enum LogType {
    Output,
    Error,
    Input,
    Console,
}

#[derive(Serialize, Deserialize)]
pub struct Log {
    pub log_type: LogType,
    pub message: String,
}

impl From<LogOutput> for Log {
    fn from(value: LogOutput) -> Self {
        let log_type = match value {
            LogOutput::StdErr { message: _ } => LogType::Error,
            LogOutput::StdOut { message: _ } => LogType::Output,
            LogOutput::StdIn { message: _ } => LogType::Input,
            LogOutput::Console { message: _ } => LogType::Console,
        };

        Self {
            log_type,
            message: value.to_string(),
        }
    }
}
