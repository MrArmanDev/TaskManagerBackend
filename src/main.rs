use std::sync::Arc;

use crate::{
    config::env,
    db::create_pool,
    repository::{TaskRepository, UserRepository},
    routers::create_router,
    services::{AuthService, TaskService}, utils::start_database_scheduler,
};

mod config;
mod db;
mod dto;
mod errors;
mod handlers;
mod middlewares;
mod models;
mod repository;
mod routers;
mod services;
mod utils;
use axum::http::Method;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::{cors::{Any, CorsLayer}, trace::TraceLayer};


#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "tower_http=info".into()), 
    )
    .with(tracing_subscriber::fmt::layer().compact()) 
    .init();
    let envs = env::EnvData::new().expect("env data fetch fail");

    let pool = create_pool(&envs.database_url)
        .await
        .expect("fail data base connection");
    println!("database connect");
    sqlx::migrate!("./migrations/")
        .run(&pool)
        .await
        .expect("fail to run migration");

    
    start_database_scheduler(pool.clone()).await;
    let user_repo = UserRepository::new(pool.clone());
    let task_repo = TaskRepository::new(pool.clone());
    let auth_service = Arc::new(AuthService::new(user_repo, envs.secret_key.clone()));
    let task_service = Arc::new(TaskService::new(task_repo));
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH])
        .allow_headers(Any);    
    let app = create_router(auth_service, task_service).await.layer(cors).layer(TraceLayer::new_for_http());
    let addr = format!("127.0.0.1:{}", envs.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("server running on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
