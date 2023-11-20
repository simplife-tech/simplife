use axum::{routing::post, Router, Extension};

pub mod login;

pub fn new_router<T: login::LoginHandlerTrait>(handler: T) -> Router {
    Router::new()
        .route("/login", post(login::user_login::<T>))
        .layer(axum::middleware::from_fn(akasha::middleware::trace_http))
        .layer(akasha::middleware::ContextLayer)
        .layer(Extension(handler))
}