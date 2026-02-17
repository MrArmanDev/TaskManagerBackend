use axum::{
    Json, extract::State, http::{HeaderMap, StatusCode, header}, response::IntoResponse
};
use std::sync::Arc;
use validator::Validate;

use crate::{
    dto::{LoginRequest, RegisterDto},
    errors::AppError,
    services::AuthService,
};

pub async fn register(
    State(auth_service): State<Arc<AuthService>>,
    Json(payload): Json<RegisterDto>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let response = auth_service.register(payload).await?;

    let mut headers = HeaderMap::new();
    let cookie = format!("token={}; HttpOnly; path=/; Max-Age=36000; SameSite=Lax", response.token);
    headers.insert(header::SET_COOKIE, header::HeaderValue::from_str(&cookie).unwrap());
    let body = serde_json::to_string_pretty(&response).unwrap_or_else(|_| format!("{:?}", response));
    println!("[REGISTER] response:\n{}", body);

    Ok((StatusCode::CREATED, headers ,Json(response)))
}


pub async fn login(
    State(auth_service): State<Arc<AuthService>>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    let response = auth_service.login(payload).await?;
    let body = serde_json::to_string_pretty(&response).unwrap_or_else(|_| format!("{:?}", response));
    let mut headers = HeaderMap::new();
    let cookie = format!("token={}; HttpOnly; path=/; Max-Age=36000; SameSite=Lax", response.token);
    headers.insert(header::SET_COOKIE, header::HeaderValue::from_str(&cookie).unwrap());


    let body = serde_json::to_string_pretty(&response).unwrap_or_else(|_| format!("{:?}", response));
    eprintln!("[LOGIN] response:\n{}", body);
    Ok((StatusCode::OK, headers ,Json(response)))
}

pub async fn health_check() -> &'static str {
    "ok"
}


