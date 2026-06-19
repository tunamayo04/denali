use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use axum::extract::Query;
use axum::http::Method;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
    .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_origin(Any);

    let service = ServiceBuilder::new()
        .layer(cors);

    let app = Router::new()
        .route("/GetBudgetRows", get(get_budget_rows))
        .layer(service);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_budget_rows(Query(params): Query<GetBudgetItemsRequest>) -> (StatusCode, Json<Vec<BudgetItem>>) {
    println!("{}", params.year);
    println!("{}", params.month);
    let rows = vec![
        BudgetItem {
            category: String::from("Rent"),
            budget: Decimal::from(1750),
            actual: Decimal::from(1750),
            color: String::from("#0084FF")
        }
    ];

    (StatusCode::OK, Json(rows))
}

#[derive(Deserialize)]
struct GetBudgetItemsRequest {
    year: u16,
    month: u16,
}

#[derive(Serialize)]
struct BudgetItem {
    category: String,
    budget: Decimal,
    actual: Decimal,
    color: String,
}