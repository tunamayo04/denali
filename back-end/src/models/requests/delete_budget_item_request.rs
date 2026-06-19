use serde::Deserialize;

#[derive(Deserialize)]
pub struct DeleteBudgetItemRequest {
    pub id: i32,
}
