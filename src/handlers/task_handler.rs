use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode, header::AUTHORIZATION},
    response::IntoResponse,
};
use std::sync::Arc;
use validator::Validate;

use crate::{
    config::env,
    dto::{CreateTaskDto, DeleteTaskDto, UpdateTaskDto},
    errors::AppError,
    services::TaskService,
    utils::{Claims, decode_jwt},
};

pub async fn create_task(
    State(task_service): State<Arc<TaskService>>,
    headers: HeaderMap,
    Json(payload): Json<CreateTaskDto>,
) -> Result<impl IntoResponse, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;
    let auth_header: &str = headers
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| -> AppError { AppError::Unauthorized })?;
    let token: &str = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| -> AppError { AppError::Unauthorized })?;
    let envs = env::EnvData::new().map_err(|e| AppError::InternalServerError(e))?;
    let claims: Claims = decode_jwt(token, &envs.secret_key).map_err(|_| AppError::Unauthorized)?;

    let user_id = claims.sub;
    let result = task_service.create_task(payload, user_id).await;
    match result {
        Ok(response) => {
            let body = serde_json::to_string_pretty(&response)
                .unwrap_or_else(|_| format!("{:?}", response));
            eprintln!("[TASK][CREATE] user_id={} status=201\n{}", user_id, body);
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(err) => {
            eprintln!("[TASK][CREATE] user_id={} status=ERR err={:?}", user_id, err);
            Err(err)
        }
    }
}

pub async fn get_task(
    State(task_service): State<Arc<TaskService>>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    let auth_header: &str = headers
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| -> AppError { AppError::Unauthorized })?;

    let token: &str = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| -> AppError { AppError::Unauthorized })?;

    let envs = env::EnvData::new().map_err(|e| AppError::InternalServerError(e))?;
    let claims: Claims = decode_jwt(token, &envs.secret_key).map_err(|_| AppError::Unauthorized)?;

    let user_id = claims.sub;
    let response = task_service.get_task(user_id).await?;
    Ok((StatusCode::OK, Json(response)))
}

pub async fn update_task(
    State(task_service): State<Arc<TaskService>>,
    headers: HeaderMap,
    Json(payload): Json<UpdateTaskDto>,
) -> Result<impl IntoResponse, AppError> {
    let auth_header: &str = headers
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| -> AppError { AppError::Unauthorized })?;

    let token: &str = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| -> AppError { AppError::Unauthorized })?;

    let envs = env::EnvData::new().map_err(|e| AppError::InternalServerError(e))?;
    let claims: Claims = decode_jwt(token, &envs.secret_key).map_err(|_| AppError::Unauthorized)?;

    let user_id = claims.sub;
    let response = task_service.update_task(user_id, payload).await?;
    Ok((StatusCode::OK, Json(response)))
}



pub async fn delete_task(
    State(task_service): State<Arc<TaskService>>,
    headers: HeaderMap,
    Json(payload): Json<DeleteTaskDto>,
) -> Result<impl IntoResponse, AppError> {
    let auth_header: &str = headers
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| -> AppError { AppError::Unauthorized })?;

    let token: &str = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| -> AppError { AppError::Unauthorized })?;
    let envs = env::EnvData::new().map_err(|e| AppError::InternalServerError(e))?;
    let claims: Claims = decode_jwt(token, &envs.secret_key).map_err(|_| AppError::Unauthorized)?;

    let user_id = claims.sub;
    task_service.delete_task(payload.id, user_id).await?;
    Ok((
        StatusCode::OK, 
        Json(serde_json::json!({ "message": format!("Task {} successfully deleted", payload.id) }))
    ))
}


