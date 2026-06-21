use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{Queryable, Selectable};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::repositories::schema::sql_types::AccountType"]
#[DbValueStyle = "PascalCase"]
pub enum AccountType {
    Checking,
    Saving,
    #[db_rename = "Credit Card"]
    CreditCard,
    Investment,
    Cash,
    Loan,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::repositories::schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub institution_name: String,
    pub account_type: AccountType,
    pub currency: String,
    pub opening_balance: BigDecimal,
    pub current_balance: BigDecimal,
    pub is_closed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
