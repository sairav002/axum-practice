#![allow(unused)]

pub use self::errors::{Error, Result};

use axum::{extract::{Path, Query}, response::{Html, IntoResponse}, routing::{get, get_service}, Router};
use serde::Deserialize;
use tower_http::services::ServeDir;

mod errors;
mod web;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .fallback_service(routes_static());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on {:?}\n", listener.local_addr());
    axum::serve(listener, routes_all).await.unwrap();

    //#[warn(while_true)]
    //loop {};
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}


#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello {name}"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("Hello2 <strong>{name}</strong>"))
}