// @generated automatically by Diesel CLI.

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
