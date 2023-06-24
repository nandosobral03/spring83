use super::route_utils::handle_error;
use crate::services::boards;
use axum::extract::Path;
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Html};
use hyper::StatusCode;

pub async fn put_board(headers: HeaderMap, Path(key): Path<String>, body: String) -> impl IntoResponse {
    // Special key to allow for testing
    if key == "ab589f4dde9fce4180fcf42c7b05185b0a02a5d682e353fa39177995083e0583"{
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }

    if body.len() > 2217 {
        return (StatusCode::PAYLOAD_TOO_LARGE, "Body too long").into_response();
    }
    let signature = match headers.get("Spring-Signature") {
        Some(sig) => sig.to_str().unwrap().to_string(),
        None => return (StatusCode::UNAUTHORIZED, "Missing signature").into_response(),
    };

    match boards::put_board(key, signature, body).await {
        Ok(_) => (StatusCode::OK, "OK").into_response(),
        Err(e) => handle_error(e).into_response(),
    }
}


pub async fn get_board(headers: HeaderMap, Path(key): Path<String>) -> impl IntoResponse {
    if key == "ab589f4dde9fce4180fcf42c7b05185b0a02a5d682e353fa39177995083e0583"{
        match boards::get_test_board().await {
            Ok(board) => {
                let mut response = (StatusCode::OK, Html(board.body)).into_response();
                response.headers_mut()
                    .insert("Last-Modified", board.last_modified.parse().unwrap());
                response.headers_mut()
                    .insert("Spring-Signature", board.signature.parse().unwrap());
                return response;
            }
            Err(e) => return handle_error(e).into_response()
        }
    }

    let last_modified = match headers.get("if-modified-since") {
        Some(date) => {
            let date = date.to_str().unwrap();
            match chrono::DateTime::parse_from_rfc3339(date) {
                Ok(date) => Some(date),
                Err(_) => return (StatusCode::BAD_REQUEST, "Date must be ISO8601 format").into_response(),
            }
        }
        None => None
    };

    match boards::get_board(&key, last_modified).await {
        Ok(board) => {
            let mut response = (StatusCode::OK, Html(board.body)).into_response();
            response.headers_mut()
                .insert("Last-Modified", board.last_modified.parse().unwrap());
            response.headers_mut()
                .insert("Spring-Signature", board.signature.parse().unwrap());
            response
        }
        Err(e) => handle_error(e).into_response(),
    }
}