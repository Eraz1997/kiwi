use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    routing::{delete, get, put},
};
use tokio::sync::Mutex;

use crate::{
    error::Error,
    managers::{dynamic_dns::DynamicDnsManager, secrets::SecretsManager},
    routes::admin::api::dynamic_dns::models::{
        EnableDynamicDnsConfigurationRequest, GetDynamicDnsConfigurationResponse,
    },
};

mod models;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(get_dynamic_dns_configuration))
        .route("/", put(enable_dynamic_dns))
        .route("/", delete(disable_dynamic_dns))
}

async fn get_dynamic_dns_configuration(
    Extension(dynamic_dns_manager): Extension<Arc<Mutex<Option<DynamicDnsManager>>>>,
) -> Json<GetDynamicDnsConfigurationResponse> {
    Json(GetDynamicDnsConfigurationResponse {
        enabled: dynamic_dns_manager.lock().await.is_some(),
    })
}

async fn enable_dynamic_dns(
    Extension(dynamic_dns_manager): Extension<Arc<Mutex<Option<DynamicDnsManager>>>>,
    Extension(secrets_manager): Extension<Arc<Mutex<SecretsManager>>>,
    Json(payload): Json<EnableDynamicDnsConfigurationRequest>,
) -> Result<(), Error> {
    let new_dynamic_dns_manager = DynamicDnsManager::new(&payload).await?;

    let mut dynamic_dns_manager = dynamic_dns_manager.lock().await;
    *dynamic_dns_manager = Some(new_dynamic_dns_manager);

    secrets_manager
        .lock()
        .await
        .set_dynamic_dns_api_configuration(Some(payload))
        .await?;

    Ok(())
}

async fn disable_dynamic_dns(
    Extension(dynamic_dns_manager): Extension<Arc<Mutex<Option<DynamicDnsManager>>>>,
    Extension(secrets_manager): Extension<Arc<Mutex<SecretsManager>>>,
) -> Result<(), Error> {
    let mut dynamic_dns_manager = dynamic_dns_manager.lock().await;
    *dynamic_dns_manager = None;

    secrets_manager
        .lock()
        .await
        .set_dynamic_dns_api_configuration(None)
        .await?;

    Ok(())
}
