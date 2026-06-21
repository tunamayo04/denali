// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "account_type"))]
    pub struct AccountType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AccountType;

    accounts (id) {
        id -> Int4,
        name -> Varchar,
        institution_name -> Varchar,
        account_type -> AccountType,
        #[max_length = 3]
        currency -> Bpchar,
        opening_balance -> Numeric,
        current_balance -> Numeric,
        is_closed -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    budget_items (id) {
        id -> Int4,
        month -> Date,
        category -> Varchar,
        budget_amount -> Numeric,
        actual_amount -> Numeric,
        color -> Varchar,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int4,
        vendor_id -> Int4,
        account_id -> Int4,
        amount -> Numeric,
        date -> Date,
    }
}

diesel::table! {
    vendors (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(transactions -> accounts (account_id));
diesel::joinable!(transactions -> vendors (vendor_id));

diesel::allow_tables_to_appear_in_same_query!(accounts, budget_items, transactions, vendors,);
