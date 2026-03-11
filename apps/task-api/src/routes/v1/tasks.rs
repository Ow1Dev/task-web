use axum::{
    Router,
    extract::State,
    http::StatusCode,
    routing::{get, put},
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
            .route("/{id}", put(update_tasks).delete(delete_tasks)),
    )
}

async fn get_tasks(State(state): State<AppState>) -> Result<AppJson<Vec<TaskResponse>>, ApiError> {
    let tasks = tasks::Entity::find()
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

async fn update_tasks() -> &'static str {
    "PUT tasks"
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
