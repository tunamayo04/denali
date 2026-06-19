use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetBudgetItemsRequest {
    pub year: i32,
    pub month: u32,
}
