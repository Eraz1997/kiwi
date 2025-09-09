use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    routing::{get, post},
};
use tokio::sync::Mutex;

use crate::{
    error::Error,
    managers::{
        lets_encrypt::{LetsEncryptManager, models::CertificateVerificationStatus},
        redis::RedisManager,
    },
    routes::admin::api::certificates::models::{
        GetCertificateInfoResponse, OrderCertificateRequest, OrderCertificateResponse,
    },
};

mod error;
mod models;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(get_certificate_info))
        .route("/", post(order_certificate))
        .route("/finalise", post(finalise_certificate_order))
}

async fn get_certificate_info(
    Extension(lets_encrypt_manager): Extension<Arc<Mutex<LetsEncryptManager>>>,
    Extension(redis_manager): Extension<RedisManager>,
) -> Result<Json<GetCertificateInfoResponse>, Error> {
    let info = lets_encrypt_manager
        .lock()
        .await
        .get_certificate_info()
        .await?;
    let new_pending_order = redis_manager
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
    Extension(lets_encrypt_manager): Extension<Arc<Mutex<LetsEncryptManager>>>,
    Extension(redis_manager): Extension<RedisManager>,
    Json(payload): Json<OrderCertificateRequest>,
) -> Result<Json<OrderCertificateResponse>, Error> {
    let order = lets_encrypt_manager
        .lock()
        .await
        .order_new_certificate(&payload.domain)
        .await?;
    redis_manager
        .set_last_certificate_order_url(&order.order_url)
        .await?;

    Ok(Json(OrderCertificateResponse {
        dns_record_name: order.dns_record_name,
        dns_record_value: order.dns_record_value,
    }))
}

async fn finalise_certificate_order(
    Extension(lets_encrypt_manager): Extension<Arc<Mutex<LetsEncryptManager>>>,
    Extension(redis_manager): Extension<RedisManager>,
) -> Result<Json<CertificateVerificationStatus>, Error> {
    let order_url = redis_manager
        .get_last_certificate_order_url()
        .await?
        .ok_or(Error::order_not_found())?;
    let verification_status = lets_encrypt_manager
        .lock()
        .await
        .finalise_and_save_certificates(&order_url.order_url)
        .await?;

    if let CertificateVerificationStatus::Success = verification_status {
        redis_manager.remove_last_certificate_order_url().await?;
    }

    Ok(Json(verification_status))
}
