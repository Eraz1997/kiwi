use fred::{
    prelude::{Client, ClientLike, Config, EventInterface, TcpConfig},
    types::Builder,
};

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

        client.init().await?;

        client.on_error(|(error, server)| async move {
            tracing::error!("{:?}: Connection error: {:?}", server, error);
            Ok(())
        });

        tracing::info!("Redis client connected to {}", DSN);

        Ok(Self { client })
    }
}
