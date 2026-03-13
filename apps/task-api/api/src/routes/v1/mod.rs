pub mod tasks;

use utoipa_axum::router::OpenApiRouter;

use crate::AppState;

pub fn config() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().nest("/v1/tasks", tasks::config())
}
