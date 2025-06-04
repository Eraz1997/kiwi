use std::ops::DerefMut;

use constants::{APPLICATION_NAME, DATABASE_NAME, HOST, MAX_POOL_SIZE, PORT};
use deadpool_postgres::{Config, ManagerConfig, Pool, PoolConfig, RecyclingMethod, Runtime};
use error::Error;
use refinery::embed_migrations;
use tokio_postgres::NoTls;

pub mod constants;
pub mod error;

#[derive(Clone)]
pub struct DbManager {
    #[allow(dead_code)] // TODO: Temporary
    connection_pool: Pool,
}

impl DbManager {
    pub async fn new(admin_username: &str, admin_password: &str) -> Result<Self, Error> {
        let mut config = Config::new();
        config.dbname = Some(DATABASE_NAME.to_string());
        config.application_name = Some(APPLICATION_NAME.to_string());
        config.host = Some(HOST.to_string());
        config.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });
        config.user = Some(admin_username.to_string());
        config.password = Some(admin_password.to_string());
        config.port = Some(PORT);
        config.pool = Some(PoolConfig::new(MAX_POOL_SIZE));

        let pool = config.create_pool(Some(Runtime::Tokio1), NoTls)?;

        embed_migrations!("migrations");
        let mut migrations_client = pool.get().await?;
        let connection = migrations_client.deref_mut().deref_mut();
        let migrations_report = migrations::runner().run_async(connection).await?;
        let applied_migrations_count = migrations_report.applied_migrations().len();

        tracing::info!(
            "db manager initialised, applied {} migrations",
            applied_migrations_count
        );
        Ok(Self {
            connection_pool: pool,
        })
    }
}
