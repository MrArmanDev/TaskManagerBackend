pub mod auth_router;
pub mod task_router;
pub use auth_router::auth_router;
pub use task_router::task_router;
use axum::Router;
use std::sync::Arc;

use crate::services::{AuthService, TaskService};

pub async fn create_router(
    auth_service: Arc<AuthService>,
    task_service: Arc<TaskService>,
) -> Router {

    Router::new()
        .nest("/auth", auth_router().with_state(auth_service))
        .nest("/task", task_router().with_state(task_service))
}
