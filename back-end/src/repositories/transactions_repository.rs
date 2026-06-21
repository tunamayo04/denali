use diesel::prelude::*;
use crate::models::transactions::*;
use crate::repositories::establish_connection;
use crate::repositories::schema::transactions::*;
use crate::repositories::schema::transactions::dsl::transactions;

pub async fn get_transactions(request: &GetTransactionsRequest) -> Vec<Transaction> {
    let connection = &mut establish_connection();
    let mut query = transactions.into_boxed::<diesel::pg::Pg>().select(Transaction::as_select());

    if let Some(filter) = request.date.clone() {
        query = query.filter(filter.apply(date));
    }

    query.load(connection).unwrap()
}