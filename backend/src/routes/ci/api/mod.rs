use axum::{Extension, Json, Router, extract::Path, routing::post};

use crate::{
    error::Error,
    managers::{
        container::{
            ContainerManager,
            models::{GithubRepository, ImageSha},
        },
        db::DbManager,
        oidc::OidcManager,
    },
    routes::ci::api::models::DeployServiceRequest,
};

mod error;
mod models;

pub fn create_router() -> Router {
    Router::new().route("/deploy/{service_name}", post(deploy_service))
}

async fn deploy_service(
    Extension(oidc_manager): Extension<OidcManager>,
    Extension(container_manager): Extension<ContainerManager>,
    Extension(db_manager): Extension<DbManager>,
    Path(service_name): Path<String>,
    Json(payload): Json<DeployServiceRequest>,
) -> Result<(), Error> {
    let service_data = db_manager
        .get_service_data(&service_name)
        .await?
        .ok_or(Error::service_not_found())?;

    let token = oidc_manager
        .validate_github_oidc_token(&payload.oidc_token)
        .await?;
    let github_repo = GithubRepository::try_from(token.repository)?;

    if token.reference != "refs/heads/main" {
        return Err(Error::invalid_branch());
    }

    match &service_data.container_configuration.github_repository {
        Some(required_repo) if *required_repo == github_repo => {}
        _ => return Err(Error::invalid_repo_for_service()),
    }

    let mut new_container_configuration = service_data.container_configuration.clone();
    let image_sha = payload.image_sha.trim_start_matches("sha246").to_string();
    new_container_configuration.image_sha = ImageSha::new(image_sha)?;

    db_manager
        .update_service(&service_data, &new_container_configuration)
        .await?;
    container_manager
        .stop_and_remove_container(&service_name)
        .await?;
    container_manager
        .start_container(&new_container_configuration)
        .await?;

    Ok(())
}
