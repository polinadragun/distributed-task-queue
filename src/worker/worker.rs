use tokio::spawn;
use redis::AsyncCommands;
use crate::config::Config;
use crate::db::queries::insert_task;
use crate::worker::processor::process_task;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn start_workers(config: Config) {
    let db_pool = PgPool::connect(&config.database_url).await.unwrap();
    for _ in 0..4 {
        let config_clone = config.clone();
        let db_pool_clone = db_pool.clone();
        spawn(async move {
            worker_loop(config_clone, db_pool_clone).await;
        });
    }
}
async fn worker_loop(config: Config, db_pool: PgPool) {
    let mut redis_conn = redis::Client::open(config.redis_url.clone())
        .unwrap()
        .get_multiplexed_async_connection()
        .await.
        unwrap();
    loop {
        let result: Option<Vec<String>> = redis_conn.brpop("task_queue", 0 as f64).await.unwrap();

        if let Some(mut result) = result {
            if result.len() == 2 {
                let task_data = result.pop().unwrap();
                let task = serde_json::from_str::<serde_json::Value>(&task_data).unwrap();
                let task_id = Uuid::parse_str(task["id"].as_str().unwrap()).unwrap();
                let result = process_task(&task).await;
                insert_task(&db_pool, &task_id, &result).await.unwrap();
            }
        }

    }

}