use fred::{
    prelude::{Client, ClientLike, Config, EventInterface, TcpConfig},
    types::Builder,
};
use std::time::Duration;
use tokio::time::sleep;

use crate::error::Error;
use crate::managers::redis::constants::{CONNECTION_TIMEOUT, DSN};

mod constants;
pub mod models;
pub mod queries;

#[derive(Clone)]
pub struct RedisManager {
    client: Client,
}

impl RedisManager {
    pub async fn new(admin_password: &str) -> Result<Self, Error> {
        let mut config = Config::from_url(DSN)?;
        config.password = Some(admin_password.to_string());

        let client = Builder::from_config(config)
            .with_connection_config(|config| {
                config.connection_timeout = CONNECTION_TIMEOUT;
                config.tcp = TcpConfig {
                    nodelay: Some(true),
                    ..Default::default()
                };
            })
            .build()?;

        for tentative_count in 1..=5 {
            tracing::info!("connecting to Redis, attempt {}/5", tentative_count);
            let connection_test_result = client.init().await;

            match connection_test_result {
                Ok(_) => break,
                Err(_) if tentative_count < 5 => {
                    sleep(Duration::from_secs(2 * tentative_count)).await;
                }
                Err(error) => {
                    return Err(error.into());
                }
            }
        }

        client.on_error(|(error, server)| async move {
            tracing::error!("{:?}: Connection error: {:?}", server, error);
            Ok(())
        });

        tracing::info!("Redis client connected to {}", DSN);

        Ok(Self { client })
    }
}
