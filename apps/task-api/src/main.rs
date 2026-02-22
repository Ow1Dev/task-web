use axum::{
    Json, Router,
    http::{HeaderValue, Method},
    routing::get,
};
use serde::Serialize;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(root)).layer(
        CorsLayer::new()
            .allow_origin("http://localhost:5000".parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET]),
    );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct Health {
    status: &'static str,
}

async fn root() -> Json<Health> {
    Json(Health { status: "Ok" })
}
