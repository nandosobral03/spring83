use std::env;

use axum::{extract::Path, response::IntoResponse};
use dotenvy::dotenv;
use hyper::{HeaderMap, StatusCode};

use crate::services::deny_list;

use super::route_utils::handle_error;

pub async fn add_denied_key(headers: HeaderMap, Path(key): Path<String>) -> impl IntoResponse {
    dotenv().ok();
    let admin_password = env::var("PASSWORD");
    if let Ok(admin_password) = admin_password {
        if let Some(password) = headers.get("authorization") {
            if password.to_str().unwrap() != admin_password {
                return (StatusCode::FORBIDDEN, "Wrong password").into_response();
            }
        } else {
            return (StatusCode::UNAUTHORIZED, "Missing password").into_response();
        }
    } else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Password not set").into_response();
    }

    match deny_list::add_denied_key(key).await {
        Ok(_) => (StatusCode::OK, "OK").into_response(),
        Err(e) => handle_error(e).into_response(),
    }
}

pub async fn delete_denied_key(headers: HeaderMap, Path(key): Path<String>) -> impl IntoResponse {
    dotenv().ok();
    let admin_password = env::var("PASSWORD");
    if let Ok(admin_password) = admin_password {
        if let Some(password) = headers.get("authorization") {
            if password.to_str().unwrap() != admin_password {
                return (StatusCode::FORBIDDEN, "Wrong password").into_response();
            }
        } else {
            return (StatusCode::UNAUTHORIZED, "Missing password").into_response();
        }
    } else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Password not set").into_response();
    }

    match deny_list::delete_denied_key(key).await {
        Ok(_) => (StatusCode::OK, "OK").into_response(),
        Err(e) => handle_error(e).into_response(),
    }
}

pub async fn get_denied_keys() -> impl IntoResponse {
    match deny_list::get_denied_keys().await {
        Ok(keys) => (StatusCode::OK, format!("{:?}", keys)).into_response(),
        Err(e) => handle_error(e).into_response(),
    }
}
