use axum::{
    Router,
    extract::{Query, State},
    routing::get,
};
use chrono::{Local, TimeDelta};
use hyper::HeaderMap;
use kangaroo_axum::{IntoKangarooError, kangarooise};

use crate::{
    constants::KIWI_USER_ID_HEADER_NAME,
    error::Error,
    managers::{
        container::ContainerManager,
        db::{DbManager, models::ServiceData},
    },
    routes::admin::models::{
        CertificateInfo, CertificatesData, DetailedServiceData, DynamicDnsData, EditServiceData,
        GenericAdminData, ServicesData, User, UsersData,
    },
    state::AppState,
};

mod api;
pub mod models;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/api", api::create_router())
        .route("/", get(get_generic_admin_page))
        .route("/users", get(get_users))
        .route("/services", get(get_services))
        .route("/services/new", get(get_generic_admin_page))
        .route("/services/edit", get(get_edit_service))
        .route("/dynamic-dns", get(get_dynamic_dns))
        .route("/certificates", get(get_certificates))
}

#[kangarooise]
async fn get_generic_admin_page(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<GenericAdminData, Error> {
    let user = get_current_user(&state.db_manager, headers).await?;
    Ok(GenericAdminData { me: user })
}

#[kangarooise]
async fn get_users(headers: HeaderMap, State(state): State<AppState>) -> Result<UsersData, Error> {
    let user = get_current_user(&state.db_manager, headers).await?;
    let users = get_users_data(&state.db_manager).await?;
    Ok(UsersData { me: user, users })
}

#[kangarooise]
async fn get_services(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<ServicesData, Error> {
    let user = get_current_user(&state.db_manager, headers).await?;
    let services = get_services_data(&state.db_manager).await?;

    Ok(ServicesData { me: user, services })
}

#[kangarooise]
async fn get_edit_service(
    headers: HeaderMap,
    Query(name): Query<String>,
    State(state): State<AppState>,
) -> Result<EditServiceData, Error> {
    let user = get_current_user(&state.db_manager, headers).await?;
    let service_data =
        get_detailed_service_data(&state.db_manager, &state.container_manager, &name).await?;
    let to_date = Local::now().naive_local();
    let from_date = Local::now().naive_local() - TimeDelta::hours(1);
    let logs = state
        .container_manager
        .get_container_logs(&name, from_date, to_date)
        .await?;

    Ok(EditServiceData {
        me: user,
        service: service_data,
        logs,
    })
}

#[kangarooise]
async fn get_dynamic_dns(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<DynamicDnsData, Error> {
    let user = get_current_user(&state.db_manager, headers).await?;
    let enabled = state.dynamic_dns_manager.lock().await.is_some();

    Ok(DynamicDnsData { me: user, enabled })
}

#[kangarooise]
async fn get_certificates(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<CertificatesData, Error> {
    let user = get_current_user(&state.db_manager, headers).await?;
    let info = get_certificate_info_data(&state).await?;

    Ok(CertificatesData {
        me: user,
        certificate: info,
    })
}

pub async fn get_current_user(db_manager: &DbManager, headers: HeaderMap) -> Result<User, Error> {
    let user_id = headers
        .get(KIWI_USER_ID_HEADER_NAME)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<i64>().ok())
        .ok_or(Error::serialisation())?;
    let user_data = db_manager
        .get_user_data_from_id(&user_id)
        .await?
        .ok_or(Error::unauthorised())?;

    Ok(User {
        username: user_data.username,
        role: user_data.role,
    })
}

pub async fn get_users_data(db_manager: &DbManager) -> Result<Vec<User>, Error> {
    let users_data = db_manager.get_users_data().await?;
    let users: Vec<User> = users_data
        .into_iter()
        .map(|user_data| User {
            username: user_data.username,
            role: user_data.role,
        })
        .collect();
    Ok(users)
}

pub async fn get_certificate_info_data(state: &AppState) -> Result<CertificateInfo, Error> {
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

    Ok(CertificateInfo {
        issuer: info.issuer,
        expiration_date: info.expiration_date,
        new_pending_order,
    })
}

pub async fn get_services_data(db_manager: &DbManager) -> Result<Vec<ServiceData>, Error> {
    let services = db_manager
        .get_services_data()
        .await?
        .into_iter()
        .map(|service| service.with_redacted_internal_secrets())
        .collect();

    Ok(services)
}

pub async fn get_detailed_service_data(
    db_manager: &DbManager,
    container_manager: &ContainerManager,
    name: &str,
) -> Result<DetailedServiceData, Error> {
    let service = db_manager
        .get_service_data(name)
        .await?
        .ok_or(Error::container_not_found())?
        .with_redacted_internal_secrets();
    let status = container_manager.get_container_status(name).await?;

    Ok(DetailedServiceData {
        general_info: service,
        status,
    })
}
