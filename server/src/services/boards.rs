use super::service_utils::{MyError, validate_key, validate_timestamp};

pub async fn put_board(sig: String, body: String) -> Result<(), MyError> {
    validate_key(&sig)?;
    validate_timestamp(&body)?;
    Ok(())
}