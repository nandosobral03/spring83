use crate::services::service_utils::MyError;
use axum::response::IntoResponse;
use hyper::StatusCode;

pub fn handle_error(err: MyError) -> impl IntoResponse {
    (StatusCode::from_u16(err.status).unwrap(), err.message)
}
