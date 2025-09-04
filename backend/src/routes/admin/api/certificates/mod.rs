use axum::{
    Extension, Json, Router,
    routing::{get, post},
};

use crate::{
    error::Error,
    managers::{
        lets_encrypt::{LetsEncryptManager, models::CertificateVerificationStatus},
        redis::RedisManager,
    },
    routes::admin::api::certificates::models::{OrderCertificateRequest, OrderCertificateResponse},
};

mod error;
mod models;

pub fn create_router() -> Router {
    Router::new()
        .route("/", post(order_certificate))
        .route("/pending", get(is_there_any_pending_order))
        .route("/finalise", post(finalise_certificate_order))
}

async fn order_certificate(
    Extension(lets_encrypt_manager): Extension<LetsEncryptManager>,
    Extension(redis_manager): Extension<RedisManager>,
    Json(payload): Json<OrderCertificateRequest>,
) -> Result<Json<OrderCertificateResponse>, Error> {
    let order = lets_encrypt_manager
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

async fn is_there_any_pending_order(
    Extension(redis_manager): Extension<RedisManager>,
) -> Result<Json<bool>, Error> {
    let order_url = redis_manager.get_last_certificate_order_url().await?;

    Ok(Json(order_url.is_some()))
}

async fn finalise_certificate_order(
    Extension(lets_encrypt_manager): Extension<LetsEncryptManager>,
    Extension(redis_manager): Extension<RedisManager>,
) -> Result<Json<CertificateVerificationStatus>, Error> {
    let order_url = redis_manager
        .get_last_certificate_order_url()
        .await?
        .ok_or(Error::order_not_found())?;
    let verification_status = lets_encrypt_manager
        .finalise_and_save_certificates(&order_url.order_url)
        .await?;

    Ok(Json(verification_status))
}
