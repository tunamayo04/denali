use axum::{routing::get, Router};
use axum::http::Method;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use crate::controllers::budget_controller::get_budget_items;

pub mod controllers;
pub mod repositories;
pub mod models;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
    .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_origin(Any);

    let service = ServiceBuilder::new()
        .layer(cors);

    let app = Router::new()
        .route("/GetBudgetRows", get(get_budget_items))
        .layer(service);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}