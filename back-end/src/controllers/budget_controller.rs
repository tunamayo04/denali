use axum::extract::Query;
use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::{delete, get, post, put};
use tracing::log;
use crate::models::budget::*;
use crate::repositories::budget_repository;

pub fn router() -> Router {
    Router::new()
        .route("/getBudgetItems", get(get_budget_items))
        .route("/addBudgetItem", post(add_budget_item))
        .route("/editBudgetItem", put(edit_budget_item))
        .route("/deleteBudgetItem", delete(delete_budget_item))
}

/// /GetBudgetItems
pub async fn get_budget_items(Query(params): Query<GetBudgetItemsRequest>) -> (StatusCode, Json<Vec<BudgetItem>>) {
    log::info!("Received get_budget_items request");

    let items = budget_repository::get_budget_items(&params).await;

    (StatusCode::OK, Json(items))
}

/// /AddBudgetItem
pub async fn add_budget_item(Json(params): Json<AddBudgetItemRequest>) -> (StatusCode, Json<()>) {
    log::info!("Received AddBudgetItem request");

    let result = budget_repository::add_budget_item(&params).await;

    match result {
        Ok(0) => (StatusCode::UNPROCESSABLE_ENTITY, Json(())),
        Ok(_) => (StatusCode::NO_CONTENT, Json(())),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(()))
    }
}

/// /EditBudgetItem
pub async fn edit_budget_item(Json(params): Json<EditBudgetItemRequest>) -> (StatusCode, Json<()>) {
    log::info!("Received EditBudgetItem request");

    let result = budget_repository::edit_budget_item(&params).await;

    match result {
        Ok(0) => (StatusCode::UNPROCESSABLE_ENTITY, Json(())),
        Ok(_) => (StatusCode::NO_CONTENT, Json(())),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(()))
    }
}

/// /DeleteBudgetItem
pub async fn delete_budget_item(Query(params): Query<DeleteBudgetItemRequest>) -> (StatusCode, Json<()>) {
    log::info!("Received DeleteBudgetItem request");

    let result = budget_repository::delete_budget_item(&params).await;

    match result {
        Ok(0) => (StatusCode::NOT_FOUND, Json(())),
        Ok(_) => (StatusCode::NO_CONTENT, Json(())),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(()))
    }
}