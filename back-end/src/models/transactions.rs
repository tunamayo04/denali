use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};
use crate::models::Filter;

// Database models
#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::repositories::schema::transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Transaction {
    pub id: i32,
    pub vendor_id: i32,
    pub amount: BigDecimal,
    pub date: NaiveDate,
}

// Response models
#[derive(Serialize, Queryable, Selectable)]
pub struct GetTransactionsResponse {
    #[diesel(select_expression = crate::repositories::schema::transactions::id)]
    pub id: i32,
    #[diesel(select_expression = crate::repositories::schema::transactions::vendor_id)]
    pub vendor_id: i32,
    #[diesel(select_expression = crate::repositories::schema::vendors::name)]
    pub vendor_name: String,
    #[diesel(select_expression = crate::repositories::schema::accounts::id)]
    pub account_id: i32,
    #[diesel(select_expression = crate::repositories::schema::accounts::name)]
    pub account_name: String,
    #[diesel(select_expression = crate::repositories::schema::transactions::amount)]
    pub amount: BigDecimal,
    #[diesel(select_expression = crate::repositories::schema::transactions::date)]
    pub date: NaiveDate,
}

#[derive(Deserialize)]
pub struct GetTransactionsRequest {
    pub vendor_id: Option<i32>,
    pub account_id: Option<i32>,
    pub amount: Option<Filter<BigDecimal>>,
    pub date: Option<Filter<NaiveDate>>,
}