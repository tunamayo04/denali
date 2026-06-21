use axum::extract::Query;
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::get;
use tracing::log;
use crate::models::transactions::*;
use crate::repositories::transactions_repository;

pub fn router() -> Router {
    Router::new()
        .route("/getTransactions", get(get_transactions))
}

/// /GetTransactions
pub async fn get_transactions(Query(params): Query<GetTransactionsRequest>) -> (StatusCode, Json<Vec<GetTransactionsResponse>>) {
    log::info!("Received get_transactions request");

    let items = transactions_repository::get_transactions(&params).await;

    (StatusCode::OK, Json(items))
}
