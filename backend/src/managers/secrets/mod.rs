use std::io::ErrorKind;

use tokio::fs::File;
use tokio::fs::create_dir_all;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

use crate::error::Error;
use crate::managers::secrets::models::Secrets;
use crate::settings::Settings;

pub mod models;

pub struct SecretsManager {
    secrets: Secrets,
}

impl SecretsManager {
    pub async fn new_with_loaded_or_created_secrets(settings: &Settings) -> Result<Self, Error> {
        let secrets_file_path = settings.secrets_file_path();
        let secrets_file = File::open(&secrets_file_path).await;

        let secrets = match secrets_file {
            Ok(mut secrets_file) => {
                let mut raw_text = vec![];
                secrets_file.read_to_end(&mut raw_text).await?;
                let raw_json = String::from_utf8(raw_text)?;
                serde_json::from_str(&raw_json)?
            }
            Err(error) if error.kind() == ErrorKind::NotFound => Secrets::default(),
            Err(error) => {
                return Err(error.into());
            }
        };

        let secrets_file_path_parts: Vec<&str> = secrets_file_path.split("/").collect();
        let config_folder_path =
            secrets_file_path_parts[..secrets_file_path_parts.len() - 1].join("/");
        create_dir_all(config_folder_path).await?;

        let mut secrets_file = File::create(&secrets_file_path).await?;
        let json_string = serde_json::to_string(&secrets)?;
        secrets_file.write_all(json_string.as_bytes()).await?;
        secrets_file.flush().await?;

        Ok(Self { secrets })
    }

    pub fn crypto_pepper(&self) -> String {
        self.secrets.crypto_pepper.get()
    }

    pub fn db_admin_username(&self) -> String {
        self.secrets.db_admin_username.get()
    }

    pub fn db_admin_password(&self) -> String {
        self.secrets.db_admin_password.get()
    }

    pub fn redis_admin_password(&self) -> String {
        self.secrets.redis_admin_password.get()
    }
}
