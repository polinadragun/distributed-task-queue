use sqlx::{PgPool, query_as};
use crate::db::models::Task;
use uuid::Uuid;

pub async fn insert_task(pool: &PgPool, id: &Uuid, data: &str) -> sqlx::Result<Task> {
    query_as!(
        Task,
        "INSERT INTO tasks (id, status, data) VALUES ($1, 'pending', $2) RETURNING *",
        id,
        data
    )
        .fetch_one(pool)
        .await
}
