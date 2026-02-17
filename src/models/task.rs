use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "PascalCase")]
#[sqlx(type_name = "task_status", rename_all = "PascalCase")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

