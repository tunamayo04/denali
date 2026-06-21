use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GetBudgetItemsRequest {
    pub year: i32,
    pub month: u32,
}

#[derive(Deserialize)]
pub struct AddBudgetItemRequest {
    pub year: i32,
    pub month: u32,
    pub category: String,
    pub budget_amount: BigDecimal,
    pub actual_amount: BigDecimal,
    pub color: String,
}

#[derive(Deserialize)]
pub struct DeleteBudgetItemRequest {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct EditBudgetItemRequest {
    pub id: i32,
    pub year: i32,
    pub month: u32,
    pub category: String,
    pub budget_amount: BigDecimal,
    pub actual_amount: BigDecimal,
    pub color: String,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::repositories::schema::budget_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BudgetItem {
    pub id: i32,
    pub month: NaiveDate,
    pub category: String,
    pub budget_amount: BigDecimal,
    pub actual_amount: BigDecimal,
    pub color: String,
}
