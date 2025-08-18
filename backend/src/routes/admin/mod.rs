use axum::extract::Path;
use axum::http::Response;
use axum::routing::get;
use axum::{Extension, Router};
use reqwest::Body;

use crate::error::Error;
use crate::managers::local_http::LocalHttpManager;
use crate::services::ServeStaticWebApp;
use crate::settings::Settings;

mod api;

pub fn create_router(settings: &Settings) -> Router {
    let router = Router::new().nest("/api", api::create_router());

    if settings.is_development() {
        router.route("/{*path}", get(forward_to_development_frontend_server))
    } else {
        router.nest_service("/", ServeStaticWebApp::new(&settings.static_files_path))
    }
}

async fn forward_to_development_frontend_server(
    local_http_manager: Extension<LocalHttpManager>,
    Path(path): Path<String>,
) -> Result<Response<Body>, Error> {
    let response = local_http_manager.get_dev_frontend_page(path).await?;
    Ok(response)
}
