use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use super::route_utils::handle_error;
use crate::services::boards;
use crate::services::service_utils::decode_jwt;
use axum::extract::{Path, Query};
use axum::http::HeaderMap;
use axum::response::{Html, IntoResponse};
use axum::Json;
use chrono::Utc;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Orientation {
    Portrait,
    Landscape,
}

impl Display for Orientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Orientation::Portrait => write!(f, "portrait"),
            Orientation::Landscape => write!(f, "landscape"),
        }
    }
}

impl From<String> for Orientation {
    fn from(s: String) -> Self {
        match s.as_str() {
            "portrait" => Orientation::Portrait,
            "landscape" => Orientation::Landscape,
            _ => Orientation::Portrait,
        }
    }
}

pub async fn put_board(
    headers: HeaderMap,
    Path(key): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    body: String,
) -> impl IntoResponse {
    // Special key to allow for testing
    if key == "ab589f4dde9fce4180fcf42c7b05185b0a02a5d682e353fa39177995083e0583" {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }

    if body.len() > 2217 {
        return (StatusCode::PAYLOAD_TOO_LARGE, "Body too long").into_response();
    }
    let signature = match headers.get("Spring-Signature") {
        Some(sig) => sig.to_str().unwrap().to_string(),
        None => return (StatusCode::UNAUTHORIZED, "Missing signature").into_response(),
    };

    let orientation = match params.get("orientation") {
        Some(orientation) => Orientation::from(orientation.to_string()),
        None => Orientation::Portrait,
    };

    match boards::put_board(key, signature, orientation, &body).await {
        Ok(_) => (StatusCode::OK, "OK").into_response(),
        Err(e) => handle_error(e).into_response(),
    }
}

pub async fn get_board(headers: HeaderMap, Path(key): Path<String>) -> impl IntoResponse {
    if key == "ab589f4dde9fce4180fcf42c7b05185b0a02a5d682e353fa39177995083e0583" {
        match boards::get_test_board().await {
            Ok(board) => {
                let mut response = (StatusCode::OK, Html(board.body)).into_response();
                response
                    .headers_mut()
                    .insert("Last-Modified", board.last_modified.parse().unwrap());
                response
                    .headers_mut()
                    .insert("Spring-Signature", board.signature.parse().unwrap());
                response.headers_mut().insert(
                    "Orientation",
                    board.orientation.to_string().parse().unwrap(),
                );

                return response;
            }
            Err(e) => return handle_error(e).into_response(),
        }
    }

    let last_modified = match headers.get("if-modified-since") {
        Some(date) => {
            let date = date.to_str().unwrap();
            match chrono::DateTime::parse_from_rfc3339(date) {
                Ok(date) => Some(date.with_timezone(&Utc)),
                Err(_) => {
                    return (StatusCode::BAD_REQUEST, "Date must be ISO8601 format").into_response()
                }
            }
        }
        None => None,
    };

    match boards::get_board(&key, last_modified).await {
        Ok(board) => {
            let mut response = (StatusCode::OK, Html(board.body)).into_response();
            response
                .headers_mut()
                .insert("Last-Modified", board.last_modified.parse().unwrap());
            response
                .headers_mut()
                .insert("Spring-Signature", board.signature.parse().unwrap());
            response.headers_mut().insert(
                "Orientation",
                board.orientation.to_string().parse().unwrap(),
            );
            response
        }
        Err(e) => handle_error(e).into_response(),
    }
}

pub async fn get_recent_boards() -> impl IntoResponse {
    match boards::get_recently_updated_boards(5, 0).await {
        Ok(board_list) => (StatusCode::OK, Json(board_list)).into_response(),
        Err(e) => handle_error(e).into_response(),
    }
}

pub async fn get_boards_count() -> impl IntoResponse {
    match boards::get_boards_count().await {
        Ok(count) => (StatusCode::OK, Json(count)).into_response(),
        Err(e) => handle_error(e).into_response(),
    }
}

pub async fn get_followed_boards(headers: HeaderMap) -> impl IntoResponse {
    let auth = match headers.get("Authorization") {
        Some(auth) => auth.to_str().unwrap().to_string(),
        None => return (StatusCode::UNAUTHORIZED, "Missing auth header").into_response(),
    };

    let decoded_jwt = decode_jwt(&auth);
    match decoded_jwt {
        Ok(decoded_jwt) => match boards::get_followed_boards(decoded_jwt.sub).await {
            Ok(board_list) => (StatusCode::OK, Json(board_list)).into_response(),
            Err(e) => handle_error(e).into_response(),
        },
        Err(e) => handle_error(e).into_response(),
    }
}
