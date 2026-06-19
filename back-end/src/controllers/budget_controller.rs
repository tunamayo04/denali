use axum::extract::Query;
use axum::http::StatusCode;
use axum::Json;
use crate::models::requests::delete_budget_item_request::DeleteBudgetItemRequest;
use crate::models::responses::budget_item::BudgetItem;
use crate::models::requests::get_budget_items_request::GetBudgetItemsRequest;
use crate::repositories::budget_repository;

/// /GetBudgetRows
pub async fn get_budget_items(Query(params): Query<GetBudgetItemsRequest>) -> (StatusCode, Json<Vec<BudgetItem>>) {
    let items = budget_repository::get_budget_items(&params).await;

    (StatusCode::OK, Json(items))
}

/// /DeleteBudgetRow
pub async fn delete_budget_item(Query(params): Query<DeleteBudgetItemRequest>) -> (StatusCode, Json<()>) {
    let result = budget_repository::delete_budget_item(&params).await;

    match result {
        Ok(0) => (StatusCode::NOT_FOUND, Json(())),
        Ok(_) => (StatusCode::NO_CONTENT, Json(())),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(()))
    }
}