use axum::{
    Json, Router,
    http::{HeaderValue, Method},
    routing::get,
};
use serde::Serialize;
use tower_http::cors::CorsLayer;

mod modules;
mod routes;

use crate::routes::not_found::not_found;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(root))
        .merge(routes::v1::config())
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_headers(tower_http::cors::Any),
        )
        .fallback(not_found);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct Health {
    status: &'static str,
}

async fn root() -> Json<Health> {
    Json(Health { status: "Ok" })
}
