use axum::routing::get;
use axum::{Extension, Json, Router};

use crate::error::Error;
use crate::managers::db::DbManager;
use crate::routes::admin::api::models::{GetUsersResponse, User};

mod models;

pub fn create_router() -> Router {
    Router::new().route("/users", get(get_users))
}

async fn get_users(
    Extension(db_manager): Extension<DbManager>,
) -> Result<Json<GetUsersResponse>, Error> {
    let users_data = db_manager.get_users_data().await?;
    let users: Vec<User> = users_data
        .into_iter()
        .map(|user_data| User {
            username: user_data.username,
            role: user_data.role,
        })
        .collect();

    Ok(Json(users))
}
