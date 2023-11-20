mod cash;
pub mod ledger;

use axum::{Router, routing::{post, get}, Extension};

pub fn new_ledger_router<T: ledger::LedgerHandlerTrait>(handler: T) -> Router {
    Router::new()
        .route("/ledger/add", post(ledger::add_ledger::<T>))
        .route("/ledger/delete", post(ledger::delete_ledger::<T>))
        .route("/ledger/list", get(ledger::ledger_list::<T>))
        .layer(axum::middleware::from_fn(akasha::middleware::login_check))
        .layer(axum::middleware::from_fn(akasha::middleware::trace_http))
        .layer(akasha::middleware::ContextLayer)
        .layer(Extension(handler))
}