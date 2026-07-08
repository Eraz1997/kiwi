use axum::Router;
use axum::extract::{Path, State};
use axum::http::Response;
use axum::routing::get;
use reqwest::Body;

use crate::error::Error;
use crate::services::ServeStaticWebApp;
use crate::settings::Settings;
use crate::state::AppState;

mod api;

pub fn create_router(settings: &Settings) -> Router<AppState> {
    let router = Router::new().nest("/api", api::create_router());

    if settings.is_development() {
        router
            .route("/", get(forward_to_development_frontend_server_root))
            .route("/{*path}", get(forward_to_development_frontend_server))
    } else {
        let static_service = ServeStaticWebApp::new(&settings.static_files_path);
        router
            .route_service("/", static_service.clone())
            .route_service("/{*path}", static_service)
    }
}

async fn forward_to_development_frontend_server_root(
    State(state): State<AppState>,
) -> Result<Response<Body>, Error> {
    let response = state
        .local_http_manager
        .get_dev_frontend_page("/".to_string())
        .await?;
    Ok(response)
}

async fn forward_to_development_frontend_server(
    State(state): State<AppState>,
    Path(path): Path<String>,
) -> Result<Response<Body>, Error> {
    let response = state.local_http_manager.get_dev_frontend_page(path).await?;
    Ok(response)
}
