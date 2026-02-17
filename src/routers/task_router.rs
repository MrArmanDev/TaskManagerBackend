use std::sync::Arc;
use axum::{Router, routing::{delete, get, post, put}};
use crate::{handlers::{create_task, get_task, delete_task, update_task}, services::TaskService};



pub fn task_router() -> Router<Arc<TaskService>> {
    Router::new()
    .route("/api/auth/create", post(create_task))
    .route("/api/auth/get", get(get_task))
    .route("/api/auth/update", put(update_task))
    .route("/api/auth/delete", delete(delete_task))
}
