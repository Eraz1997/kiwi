use std::collections::HashSet;

use crate::error::Error;
use crate::managers::container::ContainerManager;
use crate::managers::container::models::ContainerConfiguration;
use crate::managers::redis::RedisManager;
use crate::managers::secrets::models::Secret;
use crate::routes::admin::api::services::models::{
    GetLogsResponse, GetServiceResponse, GetServicesResponse,
};
use axum::extract::{Path, Query};
use axum::routing::{delete, get, post, put};
use axum::{Extension, Json, Router};
use chrono::NaiveDateTime;

use crate::managers::db::DbManager;

mod error;
mod models;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(get_services))
        .route("/{name}", get(get_service))
        .route("/{name}/logs", get(get_logs))
        .route("/", post(create_service))
        .route("/{name}", delete(delete_service))
        .route("/{previous_name}", put(edit_service))
}

async fn get_services(
    Extension(db_manager): Extension<DbManager>,
) -> Result<Json<GetServicesResponse>, Error> {
    let services = db_manager
        .get_services_data()
        .await?
        .into_iter()
        .map(|service| service.with_redacted_internal_secrets())
        .collect();
    Ok(Json(GetServicesResponse { services }))
}

async fn get_service(
    Extension(container_manager): Extension<ContainerManager>,
    Extension(db_manager): Extension<DbManager>,
    Path(name): Path<String>,
) -> Result<Json<GetServiceResponse>, Error> {
    let service = db_manager
        .get_service_data(&name)
        .await?
        .ok_or(Error::container_not_found())?
        .with_redacted_internal_secrets();
    let status = container_manager.get_container_status(&name).await?;

    Ok(Json(GetServiceResponse {
        general_info: service,
        status,
    }))
}

async fn get_logs(
    Extension(container_manager): Extension<ContainerManager>,
    Path(name): Path<String>,
    Query(from_date): Query<NaiveDateTime>,
    Query(to_date): Query<NaiveDateTime>,
) -> Result<Json<GetLogsResponse>, Error> {
    let logs = container_manager
        .get_container_logs(&name, from_date, to_date)
        .await?;

    Ok(Json(logs))
}

async fn create_service(
    Extension(container_manager): Extension<ContainerManager>,
    Extension(db_manager): Extension<DbManager>,
    Extension(redis_manager): Extension<RedisManager>,
    Json(payload): Json<ContainerConfiguration>,
) -> Result<(), Error> {
    for port in payload.exposed_ports.iter() {
        if !ContainerManager::is_local_port_free(&port.external) {
            return Err(Error::port_in_use(&port.external));
        }
    }

    let postgres_username = Secret::default().get();
    let postgres_password = Secret::default().get();
    let redis_username = Secret::default().get();
    let redis_password = Secret::default().get();

    redis_manager
        .create_user(&redis_username, &redis_password)
        .await?;
    let service = db_manager
        .create_service(
            &payload,
            &postgres_username,
            &postgres_password,
            &redis_username,
            &redis_password,
        )
        .await;

    match service {
        Ok(service) => {
            container_manager
                .start_container(&service.container_configuration)
                .await?;
            Ok(())
        }
        Err(error) => {
            redis_manager.delete_user(&redis_username).await?;
            Err(error)
        }
    }
}

async fn delete_service(
    Extension(container_manager): Extension<ContainerManager>,
    Extension(db_manager): Extension<DbManager>,
    Extension(redis_manager): Extension<RedisManager>,
    Path(name): Path<String>,
) -> Result<(), Error> {
    let service = db_manager
        .get_service_data(&name)
        .await?
        .ok_or(Error::container_not_found())?;

    container_manager.stop_and_remove_container(&name).await?;
    container_manager
        .remove_volumes(&service.container_configuration)
        .await?;
    redis_manager
        .delete_user(&service.internal_configuration.redis_username)
        .await?;
    db_manager
        .delete_service(&name, &service.internal_configuration.postgres_username)
        .await?;

    Ok(())
}

async fn edit_service(
    Extension(container_manager): Extension<ContainerManager>,
    Extension(db_manager): Extension<DbManager>,
    Path(previous_name): Path<String>,
    Json(payload): Json<ContainerConfiguration>,
) -> Result<(), Error> {
    let service = db_manager
        .get_service_data(&previous_name)
        .await?
        .ok_or(Error::container_not_found())?;

    let already_exposed_ports: HashSet<u16> = service
        .container_configuration
        .exposed_ports
        .iter()
        .map(|port| port.external)
        .collect();

    for port in payload.exposed_ports.iter() {
        if !already_exposed_ports.contains(&port.external)
            && !ContainerManager::is_local_port_free(&port.external)
        {
            return Err(Error::port_in_use(&port.external));
        }
    }

    container_manager
        .stop_and_remove_container(&previous_name)
        .await?;

    if service.container_configuration.name != payload.name {
        // volume id depends on service name
        for volume_path in service.container_configuration.stateful_volume_paths.iter() {
            if !payload.stateful_volume_paths.contains(volume_path) {
                continue;
            }
            container_manager
                .clone_volume(&service.container_configuration, &payload, volume_path)
                .await?;
        }
        container_manager
            .remove_volumes(&service.container_configuration)
            .await?;
    }

    let updated_service = db_manager.update_service(&service, &payload).await?;

    container_manager
        .start_container(&updated_service.container_configuration)
        .await?;

    Ok(())
}
