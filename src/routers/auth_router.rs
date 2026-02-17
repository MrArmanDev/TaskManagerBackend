use axum::{Router, routing::{get, post}};
use std::sync::Arc;


use crate::{
    handlers::{health_check, login, register},
    services::AuthService,
};

pub fn auth_router() -> Router<Arc<AuthService>> {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
}
