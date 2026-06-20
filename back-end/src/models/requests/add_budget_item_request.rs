use bigdecimal::BigDecimal;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddBudgetItemRequest {
    pub year: i32,
    pub month: u32,
    pub category: String,
    pub budget_amount: BigDecimal,
    pub actual_amount: BigDecimal,
    pub color: String,
}
