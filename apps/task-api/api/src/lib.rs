use std::env;

use axum::{
    Router,
    http::{HeaderValue, Method},
};
use sea_orm::Database;
use tower_http::cors::CorsLayer;

use crate::{
    openapi::{AppState, router},
    routes::not_found::not_found,
};

pub mod models;
pub mod openapi;
pub mod routes;

pub async fn run() {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    let state = AppState { conn };

    let (router, _) = router();

    let router = Router::new()
        .merge(router)
        .with_state(state)
        .fallback(not_found)
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_headers(tower_http::cors::Any),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
