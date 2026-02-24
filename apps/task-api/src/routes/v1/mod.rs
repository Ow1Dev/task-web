use axum::Router;

mod tasks;

pub fn config() -> Router {
    Router::new().nest("/v1", Router::new().merge(tasks::config()))
}
