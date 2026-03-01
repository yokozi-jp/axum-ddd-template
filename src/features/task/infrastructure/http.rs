//! Task HTTP handlers

use crate::features::task::application::CreateTaskCommand;
use crate::features::task::domain::Task;
use crate::shared::domain::entity::Entity;
use crate::shared::infrastructure::http::ApiError;
use crate::AppState;
use std::sync::Arc;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

type ApiResult<T> = Result<T, ApiError>;

/// HTTP response body for a task
#[derive(Serialize)]
pub struct TaskResponse {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

impl From<Task> for TaskResponse {
    fn from(t: Task) -> Self {
        Self {
            id: t.id().value().to_owned(),
            user_id: t.user_id().value().to_owned(),
            title: t.title().to_owned(),
            description: t.description().to_owned(),
            completed: t.is_completed(),
        }
    }
}

/// HTTP request body for creating a task
#[derive(Deserialize)]
pub struct CreateTaskRequest {
    pub user_id: String,
    pub title: String,
    pub description: String,
}

/// Query parameter for filtering tasks
#[derive(Deserialize)]
pub struct TaskQuery {
    /// Filter by user ID (optional; omit to list all tasks)
    pub user_id: Option<String>,
}

/// Task feature router
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/tasks", post(create_task).get(list_tasks))
        .route("/tasks/{id}", get(get_task).delete(delete_task))
        .route("/tasks/{id}/complete", patch(complete_task))
}

/// Create a new task
pub async fn create_task(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateTaskRequest>,
) -> ApiResult<(StatusCode, Json<TaskResponse>)> {
    let task = state
        .task
        .create
        .execute(CreateTaskCommand { user_id: body.user_id, title: body.title, description: body.description })
        .await
        .map_err(ApiError::from)?;
    Ok((StatusCode::CREATED, Json(task.into())))
}

/// Get a task by ID
pub async fn get_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> ApiResult<Json<TaskResponse>> {
    let task = state.task.get.execute(&id).await.map_err(ApiError::from)?;
    Ok(Json(task.into()))
}

/// List tasks, optionally filtered by `user_id`
pub async fn list_tasks(
    State(state): State<Arc<AppState>>,
    Query(query): Query<TaskQuery>,
) -> ApiResult<Json<Vec<TaskResponse>>> {
    let tasks = state
        .task
        .list
        .execute(query.user_id.as_deref())
        .await
        .map_err(ApiError::from)?;
    Ok(Json(tasks.into_iter().map(Into::into).collect()))
}

/// Complete a task
pub async fn complete_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> ApiResult<Json<TaskResponse>> {
    let task = state.task.complete.execute(&id).await.map_err(ApiError::from)?;
    Ok(Json(task.into()))
}

/// Delete a task by ID
pub async fn delete_task(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> ApiResult<StatusCode> {
    state.task.delete.execute(&id).await.map_err(ApiError::from)?;
    Ok(StatusCode::NO_CONTENT)
}
