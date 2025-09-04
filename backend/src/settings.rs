use std::str::FromStr;

use clap::Parser;
use instant_acme::LetsEncrypt;

use crate::error::Error;

#[derive(Parser, Debug)]
pub struct Settings {
    #[arg(long, default_value_t = default_config_folder_path())]
    config_folder_path: String,
    #[arg(long, default_value = "3000")]
    pub dev_frontend_server_port: i32,
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
    #[arg(long, default_value = "staging")]
    lets_encrypt_environment: LetsEncryptEnvironment,
    #[arg(long, default_value = "info")]
    pub log_level: tracing::Level,
    #[arg(long, default_value = "5000")]
    port: i32,
    #[arg(long, default_value = "/path")]
    pub static_files_path: String,
}

impl Settings {
    pub fn connection_string(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn is_development(&self) -> bool {
        cfg!(debug_assertions)
    }

    pub fn secrets_file_path(&self) -> String {
        format!("{}/secrets.json", self.config_folder_path)
    }

    pub fn tls_public_certificate_path(&self) -> String {
        format!("{}/tls_public_certificate.pem", self.config_folder_path)
    }

    pub fn tls_private_key_path(&self) -> String {
        format!("{}/tls_private_key.pem", self.config_folder_path)
    }

    pub fn lets_encrypt_directory_url(&self) -> String {
        match self.lets_encrypt_environment {
            LetsEncryptEnvironment::Staging => LetsEncrypt::Staging.url().to_string(),
            LetsEncryptEnvironment::Production => LetsEncrypt::Production.url().to_string(),
        }
    }
}

pub fn default_config_folder_path() -> String {
    let home_dir = dirs::home_dir()
        .and_then(|directory| directory.into_os_string().into_string().ok())
        .unwrap_or_default();
    format!("{}/.kiwi", home_dir)
}

#[derive(Debug, Clone)]
enum LetsEncryptEnvironment {
    Staging,
    Production,
}

impl FromStr for LetsEncryptEnvironment {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "staging" => Ok(Self::Staging),
            "production" => Ok(Self::Production),
            _ => Err(Error::serialisation()),
        }
    }
}
