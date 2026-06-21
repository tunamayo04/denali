use diesel::prelude::*;
use chrono::NaiveDate;
use crate::models::budget::*;
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

pub async fn add_budget_item(request: &AddBudgetItemRequest) -> QueryResult<usize> {
    let connection = &mut establish_connection();

    let date = NaiveDate::from_ymd_opt(request.year, request.month, 1).unwrap();
    let result = diesel::insert_into(budget_items)
    .values((
        month.eq(date),
        category.eq(&request.category),
        budget_amount.eq(&request.budget_amount),
        actual_amount.eq(&request.actual_amount),
        color.eq(&request.color)))
    .execute(connection);

    result
}

pub async fn edit_budget_item(request: &EditBudgetItemRequest) -> QueryResult<usize> {
    let connection = &mut establish_connection();

    let item = budget_items.filter(id.eq(request.id));
    let result = diesel::update(item)
        .set((
            category.eq(&request.category),
            budget_amount.eq(&request.budget_amount),
            actual_amount.eq(&request.actual_amount),
            color.eq(&request.color)))
        .execute(connection);

    result
}

pub async fn delete_budget_item(request: &DeleteBudgetItemRequest) -> QueryResult<usize> {
    let connection = &mut establish_connection();

    let item = budget_items.filter(id.eq(request.id));
    let result = diesel::delete(item).execute(connection);

    result
}