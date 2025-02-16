use axum::{Router, routing::post};
use crate::api::handlers::create_task_route;
use crate::config::Config;

pub fn create_routes(config: Config) -> Router {
    Router::new()
        .route("/tasks", post(create_task_route))
        .with_state(config)
}
