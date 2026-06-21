use diesel::prelude::*;
use crate::models::transactions::*;
use crate::repositories::establish_connection;
use crate::repositories::schema::*;

pub async fn get_transactions(request: &GetTransactionsRequest) -> Vec<GetTransactionsResponse> {
    let connection = &mut establish_connection();
    let mut query = transactions::table
        .inner_join(vendors::table)
        .inner_join(accounts::table)
        .into_boxed()
        .select(GetTransactionsResponse::as_select());

    if let Some(v_id) = request.vendor_id {
        query = query.filter(transactions::vendor_id.eq(v_id));
    }

    if let Some(a_id) = request.account_id {
        query = query.filter(transactions::account_id.eq(a_id));
    }

    if let Some(amount_filter) = request.amount.clone() {
        query = query.filter(amount_filter.apply(transactions::amount));
    }

    if let Some(date_filter) = request.date.clone() {
        query = query.filter(date_filter.apply(transactions::date));
    }

    query.load(connection).unwrap()
}