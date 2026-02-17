use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::TaskStatus;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTaskDto {
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
}


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateTaskDto {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[allow(dead_code)]
pub struct DeleteTaskDto {
    pub id: i32,
}
