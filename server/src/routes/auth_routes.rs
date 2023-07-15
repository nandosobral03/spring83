use axum::{extract::Path, response::IntoResponse, Json};
use hyper::{HeaderMap, StatusCode};

use crate::services::{
    auth,
    models::{SignInRequest, SignUpRequest},
    service_utils::decode_jwt,
};

use super::route_utils::handle_error;

pub async fn sign_up(Json(body): Json<SignInRequest>) -> impl IntoResponse {
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

pub async fn assign_keys_to_user(Json(body): Json<SignUpRequest>) -> impl IntoResponse {
    match auth::assign_keys_to_user(body).await {
        Ok(_) => (StatusCode::OK, "OK").into_response(),
        Err(e) => handle_error(e).into_response(),
    }
}

pub async fn remove_user_keys(Json(body): Json<SignInRequest>) -> impl IntoResponse {
    match auth::remove_user_keys(body).await {
        Ok(_) => (StatusCode::OK, "OK").into_response(),
        Err(e) => handle_error(e).into_response(),
    }
}

pub async fn follow_key(headers: HeaderMap, Path(key): Path<String>) -> impl IntoResponse {
    let auth = match headers.get("Authorization") {
        Some(auth) => auth.to_str().unwrap().to_string(),
        None => return (StatusCode::UNAUTHORIZED, "Missing auth header").into_response(),
    };
    println!("{:?}", auth);
    let decoded_jwt = decode_jwt(&auth);
    match decoded_jwt {
        Ok(decoded_jwt) => match auth::follow_key(decoded_jwt.sub, key).await {
            Ok(_) => (StatusCode::OK, "OK").into_response(),
            Err(e) => handle_error(e).into_response(),
        },
        Err(e) => handle_error(e).into_response(),
    }
}

pub async fn get_followed_keys(headers: HeaderMap) -> impl IntoResponse {
    let auth = match headers.get("Authorization") {
        Some(auth) => auth.to_str().unwrap().to_string(),
        None => return (StatusCode::UNAUTHORIZED, "Missing auth header").into_response(),
    };

    let decoded_jwt = decode_jwt(&auth);
    match decoded_jwt {
        Ok(decoded_jwt) => match auth::get_followed_keys(decoded_jwt.sub).await {
            Ok(keys) => (StatusCode::OK, Json(keys)).into_response(),
            Err(e) => handle_error(e).into_response(),
        },
        Err(e) => handle_error(e).into_response(),
    }
}

pub async fn unfollow_key(headers: HeaderMap, Path(key): Path<String>) -> impl IntoResponse {
    let auth = match headers.get("Authorization") {
        Some(auth) => auth.to_str().unwrap().to_string(),
        None => return (StatusCode::UNAUTHORIZED, "Missing auth header").into_response(),
    };

    let decoded_jwt = decode_jwt(&auth);
    match decoded_jwt {
        Ok(decoded_jwt) => match auth::unfollow_key(decoded_jwt.sub, key).await {
            Ok(_) => (StatusCode::OK, "OK").into_response(),
            Err(e) => handle_error(e).into_response(),
        },
        Err(e) => handle_error(e).into_response(),
    }
}
