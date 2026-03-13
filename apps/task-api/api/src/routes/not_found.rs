use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::models::error::ApiError;

pub async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(ApiError {
            error: "not_found",
            description: "the requested route does not exist".to_string(),
            details: None,
        }),
    )
}
