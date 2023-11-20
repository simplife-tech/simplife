pub mod schedule;

use axum::{Router, routing::{post, get}, Extension};

pub fn new_schedule_router<T: schedule::ScheduleHandlerTrait>(handler: T) -> Router {
    Router::new()
        .route("/schedule/list", get(schedule::get_schedule_list::<T>))
        .layer(axum::middleware::from_fn(akasha::middleware::login_check))
        .layer(axum::middleware::from_fn(akasha::middleware::trace_http))
        .layer(akasha::middleware::ContextLayer)
        .layer(Extension(handler))
}
