use diesel::prelude::*;
use crate::models::accounts::Account;
use crate::repositories::establish_connection;
use crate::repositories::schema::{accounts, transactions, vendors};

pub async fn get_accounts() -> Vec<Account> {
    let connection = &mut establish_connection();
    let query = accounts::table
        .into_boxed()
        .select(Account::as_select());

    query.load(connection).unwrap()
}
