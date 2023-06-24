use axum::response::IntoResponse;
use hyper::StatusCode;
use crate::services::service_utils::MyError;


pub fn handle_error(err : MyError) -> impl IntoResponse {
    (StatusCode::from_u16(err.status).unwrap(), err.message)
}