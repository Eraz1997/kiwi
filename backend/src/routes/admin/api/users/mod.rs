use axum::extract::State;
use axum::http::HeaderMap;
use axum::routing::{delete, get, post};
use axum::{Json, Router};

use crate::error::Error;
use crate::routes::admin::api::users::models::{
    CreateUserInvitationRequest, CreateUserInvitationResponse, DeleteUserRequest, GetMeResponse,
    GetUsersResponse,
};
use crate::routes::admin::{get_current_user, get_users_data};
use crate::state::AppState;

mod error;
mod models;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_users))
        .route("/", post(create_user_invitation))
        .route("/", delete(delete_user))
        .route("/me", get(get_me))
}

async fn get_users(State(state): State<AppState>) -> Result<Json<GetUsersResponse>, Error> {
    let users = get_users_data(&state.db_manager).await?;
    Ok(Json(users))
}

async fn delete_user(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<DeleteUserRequest>,
) -> Result<(), Error> {
    let user = get_current_user(&state.db_manager, headers).await?;
    if user.username == payload.username {
        Err(Error::cannot_delete_active_user())
    } else {
        state.db_manager.delete_user(&payload.username).await?;
        Ok(())
    }
}

async fn get_me(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<GetMeResponse>, Error> {
    let user = get_current_user(&state.db_manager, headers).await?;

    Ok(Json(user))
}

async fn create_user_invitation(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserInvitationRequest>,
) -> Result<Json<CreateUserInvitationResponse>, Error> {
    let user_invitation = state
        .db_manager
        .create_user_invitation(payload.role)
        .await?;

    Ok(Json(CreateUserInvitationResponse {
        invitation_id: user_invitation.id,
    }))
}
