use axum::{
    Router,
    routing::{get, put},
};
use serde::{Deserialize, Serialize};

use crate::routes::{ApiError, api_json::AppJson};

pub fn config() -> Router {
    Router::new().nest(
        "/tasks",
        Router::new()
            .route("/", get(get_tasks).post(create_tasks))
            .route("/{id}", put(update_tasks).delete(delete_tasks)),
    )
}

async fn get_tasks() -> Result<AppJson<Vec<TaskResponse>>, ApiError> {
    Ok(AppJson(vec![TaskResponse {
        id: 1,
        title: "Creating a task".to_string(),
    }]))
}

#[derive(Deserialize)]
struct CreateTaskData {
    title: String,
}

async fn create_tasks(
    AppJson(input): AppJson<CreateTaskData>,
) -> Result<AppJson<TaskResponse>, ApiError> {
    Ok(AppJson(TaskResponse {
        id: 1,
        title: input.title,
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
    id: u64,
    title: String,
}
