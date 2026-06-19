use axum::extract::Query;
use axum::http::StatusCode;
use axum::Json;
use crate::models::responses::budget_item::BudgetItem;
use crate::models::requests::get_budget_items_request::GetBudgetItemsRequest;
use crate::repositories::budget_repository;

/// /GetBudgetRows
pub async fn get_budget_items(Query(params): Query<GetBudgetItemsRequest>) -> (StatusCode, Json<Vec<BudgetItem>>) {
    let items = budget_repository::get_budget_items(&params).await;

    (StatusCode::OK, Json(items))
}