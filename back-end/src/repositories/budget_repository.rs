use diesel::prelude::*;
use chrono::NaiveDate;
use crate::models::requests::delete_budget_item_request::DeleteBudgetItemRequest;
use crate::models::responses::budget_item::BudgetItem;
use crate::models::requests::get_budget_items_request::GetBudgetItemsRequest;
use crate::repositories::establish_connection;
use crate::repositories::schema::budget_items::*;
use crate::repositories::schema::budget_items::dsl::budget_items;

pub async fn get_budget_items(request: &GetBudgetItemsRequest) -> Vec<BudgetItem> {
    let connection = &mut establish_connection();

    let date = NaiveDate::from_ymd_opt(request.year, request.month, 1).unwrap();
    let results = budget_items
        .filter(month.eq(date))
        .select(BudgetItem::as_select())
        .load(connection)
        .unwrap();

    results
}

pub async fn delete_budget_item(request: &DeleteBudgetItemRequest) -> QueryResult<usize> {
    let connection = &mut establish_connection();

    let item = budget_items.filter(id.eq(request.id));
    let result = diesel::delete(item).execute(connection);

    result
}