#![allow(unused)]

use std::net::SocketAddr;
use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Listening on {}", addr);

    let server = axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("handler_hello - hello, {:?}", params);
    let name = params.name.as_deref().unwrap_or("World");

    Html(format!("Hello, <strong> {name}!!! </strong>"))
}

async fn handler_hello2(Path(path): Path<String>) -> impl IntoResponse {
    println!("handler_hello2 - hello, {:?}", path);
    Html(format!("Hello, <strong> {name}!!! </strong>", name = path))
}
