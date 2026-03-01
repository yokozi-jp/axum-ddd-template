//! User HTTP handlers

use crate::features::user::application::{CreateUserCommand, UpdateUserCommand};
use crate::features::user::domain::User;
use crate::shared::domain::entity::Entity;
use crate::shared::infrastructure::http::ApiError;
use crate::AppState;
use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

type ApiResult<T> = Result<T, ApiError>;

/// HTTP response body for a user
#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        Self {
            id: u.id().value().to_owned(),
            name: u.name().to_owned(),
            email: u.email().value().to_owned(),
        }
    }
}

/// HTTP request body for creating a user
#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

/// HTTP request body for updating a user
#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub name: String,
    pub email: String,
}

/// User feature router
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/users", post(create_user).get(list_users))
        .route(
            "/users/{id}",
            get(get_user).put(update_user).delete(delete_user),
        )
}

/// Create a new user
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateUserRequest>,
) -> ApiResult<(StatusCode, Json<UserResponse>)> {
    let user = state
        .create_user
        .execute(CreateUserCommand { name: body.name, email: body.email })
        .await
        .map_err(ApiError::from)?;
    Ok((StatusCode::CREATED, Json(user.into())))
}

/// Get a user by ID
pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> ApiResult<Json<UserResponse>> {
    let user = state.get_user.execute(&id).await.map_err(ApiError::from)?;
    Ok(Json(user.into()))
}

/// List all users
pub async fn list_users(State(state): State<Arc<AppState>>) -> ApiResult<Json<Vec<UserResponse>>> {
    let users = state.list_users.execute().await.map_err(ApiError::from)?;
    Ok(Json(users.into_iter().map(Into::into).collect()))
}

/// Update a user
pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(body): Json<UpdateUserRequest>,
) -> ApiResult<Json<UserResponse>> {
    let user = state
        .update_user
        .execute(&id, UpdateUserCommand { name: body.name, email: body.email })
        .await
        .map_err(ApiError::from)?;
    Ok(Json(user.into()))
}

/// Delete a user by ID
pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> ApiResult<StatusCode> {
    state.delete_user.execute(&id).await.map_err(ApiError::from)?;
    Ok(StatusCode::NO_CONTENT)
}
