use axum::Router;
use sea_orm::DatabaseConnection;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::routes::{self, v1::tasks::TASK_TAG};

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = TASK_TAG, description = "Task items management API")
    )
)]
pub struct ApiDoc;

pub fn router() -> (Router<AppState>, utoipa::openapi::OpenApi) {
    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(routes::v1::config())
        .split_for_parts()
}

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}
