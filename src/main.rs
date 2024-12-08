#![allow(unused)]

pub use self::errors::{Error, Result};

use axum::{extract::{Path, Query}, http::{Method, Uri}, middleware, response::{Html, IntoResponse, Response}, routing::{get, get_service}, Json, Router};
use ctx::Ctx;
use log::log_request;
use model::ModelController;
use serde::Deserialize;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

mod ctx;
mod errors;
mod log;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize ModelController
    let mc = ModelController::new().await?;

    let routes_api = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    // Layers executed from bottom to top
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listening on {:?}\n", listener.local_addr());
    axum::serve(listener, routes_all).await.unwrap();

    Ok(())
}

async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response
) ->  Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });

            println!(" -->> client_error_body: {client_error_body}");


            (*status_code, Json(client_error_body)).into_response()
        });

    let client_error = client_status_error.unzip().1;
    log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    println!();
    error_response.unwrap_or(res)
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