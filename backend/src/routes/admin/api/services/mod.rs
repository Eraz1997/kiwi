use std::collections::HashSet;

use crate::error::Error;
use crate::managers::container::ContainerManager;
use crate::managers::container::models::ContainerConfiguration;
use crate::managers::redis::RedisManager;
use crate::managers::secrets::models::Secret;
use crate::routes::admin::api::services::models::{
    GetLogsQuery, GetLogsResponse, GetServiceResponse, GetServicesResponse,
};
use axum::extract::{Path, Query};
use axum::routing::{delete, get, post, put};
use axum::{Extension, Json, Router};
use regex::Regex;

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
        .route("/{name}", put(edit_service))
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
    Query(GetLogsQuery { from_date, to_date }): Query<GetLogsQuery>,
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
    if !ContainerManager::is_local_port_free(&payload.exposed_port.external) {
        return Err(Error::port_in_use(&payload.exposed_port.external));
    }

    let name_regex = Regex::new(r"^[a-zA-Z0-9-_]{3,32}$")?;
    if !name_regex.is_match(&payload.name) {
        return Err(Error::invalid_name());
    }

    let postgres_username = Secret::generate(32).get();
    let postgres_password = Secret::default().get();
    let redis_username = Secret::default().get();
    let redis_password = Secret::default().get();

    redis_manager
        .create_user(&redis_username, &redis_password)
        .await?;
    redis_manager.purge_service_port(&payload.name).await?;
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
    container_manager.prune_unused_images().await?;
    redis_manager
        .delete_user(&service.internal_configuration.redis_username)
        .await?;
    redis_manager.purge_service_port(&name).await?;
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

    if service.container_configuration.name != payload.name {
        return Err(Error::inconsistent_name());
    }
    if service.container_configuration.exposed_port.external != payload.exposed_port.external {
        return Err(Error::inconsistent_port());
    }

    let updated_service = db_manager.update_service(&service, &payload).await?;

    container_manager
        .stop_and_remove_container(&previous_name)
        .await?;

    let new_volumes: HashSet<String> = payload.stateful_volume_paths.clone().into_iter().collect();
    let volumes_to_remove: Vec<String> = service
        .container_configuration
        .stateful_volume_paths
        .clone()
        .into_iter()
        .filter(|path| !new_volumes.contains(path))
        .collect();
    let mut configuration_with_volumes_to_remove = service.container_configuration.clone();
    configuration_with_volumes_to_remove.stateful_volume_paths = volumes_to_remove;
    container_manager
        .remove_volumes(&configuration_with_volumes_to_remove)
        .await?;

    container_manager
        .start_container(&updated_service.container_configuration)
        .await?;
    container_manager.prune_unused_images().await?;

    Ok(())
}
