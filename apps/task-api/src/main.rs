use axum::{
    Router,
    http::{HeaderValue, Method},
};
use tower_http::cors::CorsLayer;

mod modules;
mod routes;

use crate::routes::not_found::not_found;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5000".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_headers(tower_http::cors::Any),
        )
        .merge(routes::v1::config())
        .fallback(not_found);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
