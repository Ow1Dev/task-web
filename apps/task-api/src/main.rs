use std::env;

use axum::http::{HeaderValue, Method};
use sea_orm::{Database, DatabaseConnection};
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_redoc::{Redoc, Servable};

mod models;
mod routes;

use crate::routes::{not_found::not_found, v1::tasks::TASK_TAG};

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = TASK_TAG, description = "Task items management API")
        )
    )]
    struct ApiDoc;

    dotenv::from_filename(".env.local").ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    let state = AppState { conn };

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_headers(tower_http::cors::Any),
        )
        .merge(routes::v1::config()) // use merge, not nest
        .with_state(state)
        .fallback(not_found)
        .split_for_parts();

    let router = router.merge(Redoc::with_url("/redoc", api));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}
