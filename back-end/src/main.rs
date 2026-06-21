use axum::{Router};
use axum::http::Method;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use crate::controllers::{budget_controller, transactions_controller};

pub mod controllers;
pub mod repositories;
pub mod models;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
    .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_origin(Any)
    .allow_headers(vec![axum::http::header::CONTENT_TYPE]);

    let service = ServiceBuilder::new()
        .layer(cors);

    let app = Router::new()
        .nest("/budget", budget_controller::router())
        .nest("/transactions", transactions_controller::router())
        .layer(service);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}