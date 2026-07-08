use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};

use crate::{
    error::Error,
    managers::lets_encrypt::models::CertificateVerificationStatus,
    routes::admin::api::certificates::models::{
        GetCertificateInfoResponse, OrderCertificateRequest, OrderCertificateResponse,
    },
    state::AppState,
};

mod error;
mod models;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_certificate_info))
        .route("/", post(order_certificate))
        .route("/finalise", post(finalise_certificate_order))
}

async fn get_certificate_info(
    State(state): State<AppState>,
) -> Result<Json<GetCertificateInfoResponse>, Error> {
    let info = state
        .lets_encrypt_manager
        .lock()
        .await
        .get_certificate_info()
        .await?;
    let new_pending_order = state
        .redis_manager
        .get_last_certificate_order_url()
        .await?
        .is_some();

    Ok(Json(GetCertificateInfoResponse {
        issuer: info.issuer,
        expiration_date: info.expiration_date,
        new_pending_order,
    }))
}

async fn order_certificate(
    State(state): State<AppState>,
    Json(payload): Json<OrderCertificateRequest>,
) -> Result<Json<OrderCertificateResponse>, Error> {
    let order = state
        .lets_encrypt_manager
        .lock()
        .await
        .order_new_certificate(&payload.domain)
        .await?;
    state
        .redis_manager
        .set_last_certificate_order_url(&order.order_url)
        .await?;

    Ok(Json(OrderCertificateResponse {
        dns_record_name: order.dns_record_name,
        dns_record_value: order.dns_record_value,
    }))
}

async fn finalise_certificate_order(
    State(state): State<AppState>,
) -> Result<Json<CertificateVerificationStatus>, Error> {
    let order_url = state
        .redis_manager
        .get_last_certificate_order_url()
        .await?
        .ok_or(Error::order_not_found())?;
    let verification_status = state
        .lets_encrypt_manager
        .lock()
        .await
        .finalise_and_save_certificates(&order_url.order_url)
        .await?;

    if let CertificateVerificationStatus::Success = verification_status {
        state
            .redis_manager
            .remove_last_certificate_order_url()
            .await?;
    }

    Ok(Json(verification_status))
}
