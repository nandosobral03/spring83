use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde::Deserialize;

use crate::services::auth;

use super::route_utils::handle_error;

#[derive(Deserialize)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    pub private_key: String,
}

#[derive(Deserialize)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

pub async fn sign_up(Json(body): Json<SignUpRequest>) -> impl IntoResponse {
    match auth::sign_up(body).await {
        Ok(_) => (StatusCode::OK, "OK").into_response(),
        Err(e) => handle_error(e).into_response(),
    }
}

pub async fn sign_in(Json(body): Json<SignInRequest>) -> impl IntoResponse {
    match auth::sign_in(body).await {
        Ok(token) => (StatusCode::OK, token).into_response(),
        Err(e) => handle_error(e).into_response(),
    }
}

pub async fn remove_user(Json(body): Json<SignInRequest>) -> impl IntoResponse {
    match auth::remove_user(body).await {
        Ok(_) => (StatusCode::OK, "OK").into_response(),
        Err(e) => handle_error(e).into_response(),
    }
}
