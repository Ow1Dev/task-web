use std::env;

use axum::{
    Router,
    http::{HeaderValue, Method},
};
use sea_orm::{Database, DatabaseConnection};
use tower_http::cors::CorsLayer;

mod models;
mod routes;

use crate::routes::not_found::not_found;

#[tokio::main]
async fn main() {
    dotenv::from_filename(".env.local").ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    let state = AppState { conn };

    let app = Router::new()
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_headers(tower_http::cors::Any),
        )
        .merge(routes::v1::config())
        .with_state(state)
        .fallback(not_found);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}
