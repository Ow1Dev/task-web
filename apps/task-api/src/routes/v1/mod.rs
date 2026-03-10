use axum::Router;

use crate::AppState;

mod tasks;

pub fn config() -> Router<AppState> {
    Router::new().nest("/v1", Router::new().merge(tasks::config()))
}
