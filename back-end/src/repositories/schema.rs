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

diesel::table! {
    transactions (id) {
        id -> Int4,
        vendor_id -> Int4,
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

diesel::joinable!(transactions -> vendors (vendor_id));

diesel::allow_tables_to_appear_in_same_query!(budget_items, transactions, vendors,);
