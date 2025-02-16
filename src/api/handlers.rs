use axum::{Json, extract::State};
use serde_json::json;
use crate::config::Config;
use uuid::Uuid;
use redis::AsyncCommands;

pub async fn create_task_route(
    State(config): State<Config>,
    Json(payload): Json<serde_json::Value>
) -> Json<serde_json::Value> {
    let task_id = Uuid::new_v4().to_string();
    let serialized_task = serde_json::to_string(&json!({
        "id": task_id,
        "payload": payload
    })).unwrap();

    let mut redis_conn = redis::Client::open(config.redis_url.clone())
        .unwrap()
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let _: () = redis_conn.lpush("task_queue", serialized_task).await.unwrap();

    Json(json!({ "task_id": task_id }))
}
