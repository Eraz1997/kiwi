use axum::{
    Extension, Json, Router,
    extract::{Multipart, Path},
    routing::post,
};

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
    Router::new()
        .route("/deploy/{service_name}", post(deploy_service))
        .route("/push-tarball", post(push_tarball))
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
    container_manager
        .create_and_attach_network_for_container(&new_container_configuration)
        .await?;

    Ok(())
}

async fn push_tarball(
    Extension(oidc_manager): Extension<OidcManager>,
    Extension(container_manager): Extension<ContainerManager>,
    Extension(db_manager): Extension<DbManager>,
    mut multipart: Multipart,
) -> Result<(), Error> {
    let mut oidc_token: Option<String> = None;
    let mut tarball: Option<Vec<u8>> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| Error::invalid_ci_payload("malformed multipart payload"))?
    {
        match field.name() {
            Some("oidc_token") => {
                oidc_token = Some(
                    field
                        .text()
                        .await
                        .map_err(|_| Error::invalid_ci_payload("cannot parse oidc_token field"))?,
                );
            }
            Some("tarball") => {
                tarball = Some(
                    field
                        .bytes()
                        .await
                        .map_err(|_| Error::invalid_ci_payload("cannot parse tarball field"))?
                        .to_vec(),
                );
            }
            _ => {}
        }
    }

    let oidc_token =
        oidc_token.ok_or(Error::invalid_ci_payload("missing oidc_token form field"))?;
    let tarball = tarball.ok_or(Error::invalid_ci_payload("missing tarball form field"))?;
    if tarball.is_empty() {
        return Err(Error::invalid_ci_payload("tarball cannot be empty"));
    }

    let token = oidc_manager.validate_github_oidc_token(&oidc_token).await?;
    let github_repo = GithubRepository::try_from(token.repository)?;

    if token.reference != "refs/heads/main" {
        return Err(Error::invalid_branch());
    }

    let services = db_manager.get_services_data().await?;
    let is_authorised_repo = services.iter().any(|service| {
        service
            .container_configuration
            .github_repository
            .as_ref()
            .is_some_and(|repo| *repo == github_repo)
    });
    if !is_authorised_repo {
        return Err(Error::invalid_repo_for_service());
    }

    container_manager.load_image_tarball(tarball).await?;

    Ok(())
}
