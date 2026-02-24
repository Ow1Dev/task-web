use axum::{
    Router,
    routing::{get, put},
};

use crate::routes::ApiError;

pub fn config() -> Router {
    Router::new().nest(
        "/tasks",
        Router::new()
            .route("/", get(get_tasks).post(create_tasks))
            .route("/{id}", put(update_tasks).delete(delete_tasks)),
    )
}

async fn get_tasks() -> Result<&'static str, ApiError> {
    Err(ApiError::NotFound)
}

async fn create_tasks() -> &'static str {
    "POST tasks"
}

async fn update_tasks() -> &'static str {
    "PUT tasks"
}

async fn delete_tasks() -> &'static str {
    "DELETE tasks"
}
