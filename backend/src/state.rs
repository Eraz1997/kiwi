use std::sync::Arc;

use tokio::sync::Mutex;

use crate::managers::{
    container::ContainerManager, crypto::CryptoManager, db::DbManager,
    dynamic_dns::DynamicDnsManager, lets_encrypt::LetsEncryptManager, local_http::LocalHttpManager,
    oidc::OidcManager, redis::RedisManager, secrets::SecretsManager,
};

#[derive(Clone)]
pub struct AppState {
    pub db_manager: DbManager,
    pub container_manager: ContainerManager,
    pub crypto_manager: CryptoManager,
    pub redis_manager: RedisManager,
    pub local_http_manager: LocalHttpManager,
    pub oidc_manager: OidcManager,
    pub dynamic_dns_manager: Arc<Mutex<Option<DynamicDnsManager>>>,
    pub secrets_manager: Arc<Mutex<SecretsManager>>,
    pub lets_encrypt_manager: Arc<Mutex<LetsEncryptManager>>,
}
