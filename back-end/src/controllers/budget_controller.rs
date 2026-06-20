use axum::extract::Query;
use axum::http::StatusCode;
use axum::Json;
use tracing::log;
use crate::models::requests::add_budget_item_request::AddBudgetItemRequest;
use crate::models::requests::delete_budget_item_request::DeleteBudgetItemRequest;
use crate::models::requests::edit_budget_item_request::EditBudgetItemRequest;
use crate::models::responses::budget_item::BudgetItem;
use crate::models::requests::get_budget_items_request::GetBudgetItemsRequest;
use crate::repositories::budget_repository;

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