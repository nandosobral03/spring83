use super::route_utils::handle_error;
use crate::services::boards;
use axum::extract::Path;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use hyper::StatusCode;

// html body
pub async fn put_board(headers: HeaderMap, Path(key): Path<String>, body: String) -> impl IntoResponse {
    if body.len() > 2217 {
        return (StatusCode::BAD_REQUEST, "Body too long").into_response();
    }
    let signature = match headers.get("Spring-Signature") {
        Some(sig) => sig.to_str().unwrap().to_string(),
        None => return (StatusCode::BAD_REQUEST, "Missing signature").into_response(),
    };

    match boards::put_board(key, signature, body).await {
        Ok(_) => (StatusCode::OK, "OK").into_response(),
        Err(e) => handle_error(e).into_response(),
    }
}
