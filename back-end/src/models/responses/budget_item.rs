use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::{Queryable, Selectable};
use serde::Serialize;

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
