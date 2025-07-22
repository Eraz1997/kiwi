use axum::extract::Path;
use axum::http::Response;
use axum::routing::get;
use axum::{Extension, Router};
use reqwest::Body;

use crate::error::Error;
use crate::managers::dev_frontend::DevFrontendManager;
use crate::services::ServeStaticWebApp;
use crate::settings::Settings;

mod api;

pub fn create_router(settings: &Settings) -> Router {
    let router = Router::new().nest("/api", api::create_router());

    if settings.is_development() {
        router.route("/{*path}", get(forward_to_development_frontend_server))
    } else {
        let home_path = settings.get_home_dir();
        let config_folder_path = format!("{}/.kiwi", home_path);
        let public_assets_file_path = format!("{}/public", config_folder_path);
        router.nest_service("/", ServeStaticWebApp::new(&public_assets_file_path))
    }
}

async fn forward_to_development_frontend_server(
    dev_frontend_manager: Extension<DevFrontendManager>,
    Path(path): Path<String>,
) -> Result<Response<Body>, Error> {
    let response = dev_frontend_manager.get(path).await?;
    Ok(response)
}
