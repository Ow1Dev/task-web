use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use entity::tasks;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    AppState,
    routes::{ApiError, api_json::AppJson},
};

pub const TASK_TAG: &str = "tasks";

pub fn config() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_tasks, create_tasks))
        .routes(routes!(update_tasks, delete_tasks))
}

#[utoipa::path(
    get,
    path = "/",
    tag = TASK_TAG,
    responses(
        (status = 200, description = "Get all tasks", body = [TaskResponse])
    )
)]
async fn get_tasks(State(state): State<AppState>) -> Result<AppJson<Vec<TaskResponse>>, ApiError> {
    let tasks = tasks::Entity::find()
        .order_by_id(sea_orm::Order::Asc)
        .all(&state.conn)
        .await?
        .iter()
        .map(|t| TaskResponse {
            id: t.id,
            title: t.title.to_owned(),
            description: t.description.to_owned(),
        })
        .collect();

    Ok(AppJson(tasks))
}

#[derive(Deserialize, ToSchema)]
struct CreateTaskData {
    title: String,
    description: String,
}

#[utoipa::path(
    post,
    path = "/",
    tag = TASK_TAG,
    request_body = CreateTaskData,
    responses(
        (status = 201, description = "Task created", body = TaskResponse),
        (status = 500, description = "Internal server error")
    )
)]
async fn create_tasks(
    State(state): State<AppState>,
    AppJson(input): AppJson<CreateTaskData>,
) -> Result<(StatusCode, AppJson<TaskResponse>), ApiError> {
    let task = tasks::ActiveModel {
        title: Set(input.title),
        description: Set(input.description),
        ..Default::default()
    };

    let task: tasks::Model = task.insert(&state.conn).await?;

    Ok(AppJson::created(TaskResponse {
        id: task.id,
        title: task.title,
        description: task.description,
    }))
}

#[derive(Deserialize, ToSchema)]
struct UpdateTaskData {
    title: Option<String>,
    description: Option<String>,
}

#[utoipa::path(
    patch,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "Task ID")
    ),
    tag = TASK_TAG,
    request_body = UpdateTaskData,
    responses(
        (status = 200, description = "Task updated", body = TaskResponse),
        (status = 404, description = "Task not found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn update_tasks(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    AppJson(input): AppJson<UpdateTaskData>,
) -> Result<(StatusCode, AppJson<TaskResponse>), ApiError> {
    let mut task: tasks::ActiveModel = tasks::Entity::find_by_id(id)
        .one(&state.conn)
        .await?
        .ok_or(ApiError::NotFound)?
        .into();

    if let Some(title) = input.title {
        task.title = Set(title);
    }

    if let Some(description) = input.description {
        task.description = Set(description);
    }

    let updated_task: tasks::Model = task.update(&state.conn).await?;

    Ok(AppJson::ok(TaskResponse {
        id,
        title: updated_task.title,
        description: updated_task.description,
    }))
}

#[utoipa::path(
    delete,
    path = "/{id}",
    params(
        ("id" = i32, Path, description = "Task ID")
    ),
    tag = TASK_TAG,
    responses(
        (status = 204, description = "Task deleted"),
        (status = 404, description = "Task not found"),
        (status = 500, description = "Internal server error")
    )
)]
async fn delete_tasks(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let task: tasks::ActiveModel = tasks::Entity::find_by_id(id)
        .one(&state.conn)
        .await?
        .ok_or(ApiError::NotFound)?
        .into();

    let res = task.delete(&state.conn).await?;

    if res.rows_affected == 1 {
        return Ok(StatusCode::NO_CONTENT);
    }

    Err(ApiError::Internal(
        "Something went worng when deleting".to_string(),
    ))
}

#[derive(Serialize, ToSchema)]
struct TaskResponse {
    id: i32,
    title: String,
    description: String,
}
