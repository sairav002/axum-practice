#![allow(unused)]

use axum::{response::{Html, IntoResponse}, routing::get, Router};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(handler_hello),
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on {:?}\n", listener.local_addr());
    axum::serve(listener, routes_hello);
}


async fn handler_hello() -> impl IntoResponse {
    Html("Hello world!")
}