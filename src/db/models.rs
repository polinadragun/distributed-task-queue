use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub status: String,
    pub data: String,
}
