use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::{get, post};
use tracing::log;
use crate::models::accounts::{Account, AddAccountRequest};
use crate::repositories::accounts_repository;

pub fn router() -> Router {
    Router::new()
        .route("/getAccounts", get(get_accounts))
        .route("/addAccount", post(add_account))
}

/// /GetAccounts
pub async fn get_accounts() -> (StatusCode, Json<Vec<Account>>) {
    log::info!("Received get_accounts request");

    let items = accounts_repository::get_accounts().await;

    (StatusCode::OK, Json(items))
}

/// /AddAccount
pub async fn add_account(Json(params): Json<AddAccountRequest>) -> (StatusCode, Json<()>) {
    log::info!("Received add_account request");
    let result = accounts_repository::add_account(&params).await;

    match result {
        Ok(0) => (StatusCode::UNPROCESSABLE_ENTITY, Json(())),
        Ok(_) => (StatusCode::NO_CONTENT, Json(())),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(()))
    }
}