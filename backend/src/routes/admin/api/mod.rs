use axum::http::HeaderMap;
use axum::routing::{delete, get};
use axum::{Extension, Json, Router};

use crate::constants::KIWI_USER_ID_HEADER_NAME;
use crate::error::Error;
use crate::managers::db::DbManager;
use crate::routes::admin::api::models::{DeleteUserRequest, GetMeResponse, GetUsersResponse, User};

mod error;
mod models;

pub fn create_router() -> Router {
    Router::new()
        .route("/users", get(get_users))
        .route("/user", delete(delete_user))
        .route("/me", get(get_me))
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

async fn delete_user(
    Extension(db_manager): Extension<DbManager>,
    headers: HeaderMap,
    Json(payload): Json<DeleteUserRequest>,
) -> Result<(), Error> {
    let user = get_current_user(&db_manager, headers).await?;
    if user.username == payload.username {
        Err(Error::cannot_delete_active_user())
    } else {
        db_manager.delete_user(&payload.username).await?;
        Ok(())
    }
}

async fn get_me(
    Extension(db_manager): Extension<DbManager>,
    headers: HeaderMap,
) -> Result<Json<GetMeResponse>, Error> {
    let user = get_current_user(&db_manager, headers).await?;

    Ok(Json(user))
}

async fn get_current_user(db_manager: &DbManager, headers: HeaderMap) -> Result<User, Error> {
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
