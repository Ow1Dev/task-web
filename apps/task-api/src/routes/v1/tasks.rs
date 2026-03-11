use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch},
};
use entity::tasks;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    routes::{ApiError, api_json::AppJson},
};

pub fn config() -> Router<AppState> {
    Router::new().nest(
        "/tasks",
        Router::new()
            .route("/", get(get_tasks).post(create_tasks))
            .route("/{id}", patch(update_tasks).delete(delete_tasks)),
    )
}

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

#[derive(Deserialize)]
struct CreateTaskData {
    title: String,
    description: String,
}

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

#[derive(Deserialize)]
struct UpdateTaskData {
    title: Option<String>,
    description: Option<String>,
}

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

async fn delete_tasks() -> Result<&'static str, ApiError> {
    Err(ApiError::NotFound)
}

#[derive(Serialize)]
struct TaskResponse {
    id: i32,
    title: String,
    description: String,
}
