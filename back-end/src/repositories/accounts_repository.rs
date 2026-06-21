use diesel::prelude::*;
use crate::models::accounts::{Account, AddAccountRequest};
use crate::repositories::establish_connection;
use crate::repositories::schema::{accounts, transactions, vendors};

pub async fn get_accounts() -> Vec<Account> {
    let connection = &mut establish_connection();
    let query = accounts::table
        .into_boxed()
        .select(Account::as_select());

    query.load(connection).unwrap()
}

pub async fn add_account(request: &AddAccountRequest) -> QueryResult<usize> {
    let connection = &mut establish_connection();

    diesel::insert_into(accounts::table)
        .values((
            accounts::name.eq(&request.name),
            accounts::institution_name.eq(&request.institution_name),
            accounts::account_type.eq(&request.account_type),
            accounts::currency.eq(&request.currency),
            accounts::opening_balance.eq(&request.opening_balance),
            accounts::current_balance.eq(&request.current_balance),
            accounts::is_closed.eq(&request.is_closed),
            ))
        .execute(connection)
}