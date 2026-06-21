use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use diesel::{Queryable, Selectable};
use serde::{Deserialize, Serialize};
use crate::models::Filter;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::repositories::schema::transactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Transaction {
    pub id: i32,
    pub vendor_id: i32,
    pub amount: BigDecimal,
    pub date: NaiveDate,
}

#[derive(Deserialize)]
pub struct GetTransactionsRequest {
    pub vendor_id: Option<i32>,
    pub amount: Option<Filter<BigDecimal>>,
    pub date: Option<Filter<NaiveDate>>,
}
