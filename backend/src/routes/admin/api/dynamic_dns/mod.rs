use axum::{
    Json, Router,
    extract::State,
    routing::{delete, get, put},
};

use crate::{
    error::Error,
    managers::dynamic_dns::DynamicDnsManager,
    routes::admin::api::dynamic_dns::models::{
        EnableDynamicDnsConfigurationRequest, GetDynamicDnsConfigurationResponse,
    },
    state::AppState,
};

mod models;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_dynamic_dns_configuration))
        .route("/", put(enable_dynamic_dns))
        .route("/", delete(disable_dynamic_dns))
}

async fn get_dynamic_dns_configuration(
    State(state): State<AppState>,
) -> Json<GetDynamicDnsConfigurationResponse> {
    Json(GetDynamicDnsConfigurationResponse {
        enabled: state.dynamic_dns_manager.lock().await.is_some(),
    })
}

async fn enable_dynamic_dns(
    State(state): State<AppState>,
    Json(payload): Json<EnableDynamicDnsConfigurationRequest>,
) -> Result<(), Error> {
    let new_dynamic_dns_manager = DynamicDnsManager::new(&payload).await?;

    let mut dynamic_dns_manager = state.dynamic_dns_manager.lock().await;
    *dynamic_dns_manager = Some(new_dynamic_dns_manager);

    state
        .secrets_manager
        .lock()
        .await
        .set_dynamic_dns_api_configuration(Some(payload))
        .await?;

    Ok(())
}

async fn disable_dynamic_dns(State(state): State<AppState>) -> Result<(), Error> {
    let mut dynamic_dns_manager = state.dynamic_dns_manager.lock().await;
    *dynamic_dns_manager = None;

    state
        .secrets_manager
        .lock()
        .await
        .set_dynamic_dns_api_configuration(None)
        .await?;

    Ok(())
}
