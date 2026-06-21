use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::get;
use tracing::log;
use crate::models::accounts::Account;
use crate::repositories::accounts_repository;

pub fn router() -> Router {
    Router::new()
        .route("/getAccounts", get(get_accounts))
}

/// /GetAccounts
pub async fn get_accounts() -> (StatusCode, Json<Vec<Account>>) {
    log::info!("Received get_accounts request");

    let items = accounts_repository::get_accounts().await;

    (StatusCode::OK, Json(items))
}
