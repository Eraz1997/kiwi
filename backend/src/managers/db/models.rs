use chrono::NaiveDateTime;
use postgres_types::Json;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use uuid::Uuid;

use crate::error::Error;
use crate::managers::container::models::{
    ContainerConfiguration, EnvironmentVariable, ExposedPort, GithubRepository, ImageSha,
};
use crate::models::UserRole;

pub struct UserData {
    pub id: i64,
    pub password_hash: String,
    pub role: UserRole,
    pub username: String,
}

impl TryFrom<Row> for UserData {
    type Error = Error;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.try_get("id")?,
            password_hash: value.try_get("password_hash")?,
            role: value.try_get("role")?,
            username: value.try_get("username")?,
        })
    }
}

pub struct UserInvitation {
    pub id: Uuid,
    pub role: UserRole,
}

impl TryFrom<Row> for UserInvitation {
    type Error = Error;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.try_get("id")?,
            role: value.try_get("role")?,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct InternalServiceConfiguration {
    pub redis_username: String,
    pub postgres_username: String,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceData {
    pub container_configuration: ContainerConfiguration,
    pub created_at: NaiveDateTime,
    pub last_modified_at: NaiveDateTime,
    pub last_deployed_at: NaiveDateTime,
    pub internal_configuration: InternalServiceConfiguration,
}

impl TryFrom<Row> for ServiceData {
    type Error = Error;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        let exposed_port_vec = value.try_get::<&str, Vec<i32>>("exposed_port")?;
        let exposed_port: ExposedPort = ExposedPort {
            internal: *(exposed_port_vec.first().ok_or(Error::serialisation())?) as u16,
            external: *(exposed_port_vec.get(1).ok_or(Error::serialisation())?) as u16,
        };

        let postgres_username: String = value.try_get("postgres_username")?;
        let postgres_password: String = value.try_get("postgres_password")?;
        let redis_username: String = value.try_get("redis_username")?;
        let redis_password: String = value.try_get("redis_password")?;
        let redis_prefix = format!("{}:", redis_username);
        let github_repository =
            if let Some(repo) = value.try_get::<&str, Option<String>>("github_repository")? {
                Some(GithubRepository::try_from(repo)?)
            } else {
                None
            };

        Ok(Self {
            container_configuration: ContainerConfiguration {
                name: value.try_get("name")?,
                image_name: value.try_get("image_name")?,
                image_sha: ImageSha::new(value.try_get("image_sha")?)?,
                exposed_port,
                environment_variables: value
                    .try_get::<&str, Json<Vec<EnvironmentVariable>>>("environment_variables")?
                    .0,
                secrets: value
                    .try_get::<&str, Json<Vec<EnvironmentVariable>>>("secrets")?
                    .0,
                internal_secrets: vec![
                    EnvironmentVariable {
                        name: "KIWI_POSTGRES_URI".to_string(),
                        value: format!(
                            "psql://{}:{}@kiwi-postgres:5432/{}",
                            postgres_username, postgres_password, postgres_username
                        ),
                    },
                    EnvironmentVariable {
                        name: "KIWI_REDIS_URI".to_string(),
                        value: format!(
                            "redis://{}:{}:kiwi-redis:6379",
                            redis_username, redis_password
                        ),
                    },
                    EnvironmentVariable {
                        name: "KIWI_REDIS_PREFIX".to_string(),
                        value: redis_prefix,
                    },
                ],
                stateful_volume_paths: value.try_get("stateful_volume_paths")?,
                github_repository,
                required_role: value.try_get("required_role")?,
            },
            created_at: value.try_get("created_at")?,
            last_modified_at: value.try_get("last_modified_at")?,
            last_deployed_at: value.try_get("last_deployed_at")?,
            internal_configuration: InternalServiceConfiguration {
                redis_username,
                postgres_username,
            },
        })
    }
}

impl ServiceData {
    pub fn with_redacted_internal_secrets(mut self) -> Self {
        self.container_configuration.internal_secrets = vec![];
        self.internal_configuration = InternalServiceConfiguration {
            redis_username: String::new(),
            postgres_username: String::new(),
        };
        self
    }
}
