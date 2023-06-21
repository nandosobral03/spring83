use super::service_utils::{MyError, validate_key, validate_timestamp, validate_signature};

pub async fn put_board(key: String, sig: String, body: String) -> Result<(), MyError> {
    validate_key(&key)?;
    validate_timestamp(&body)?;
    validate_signature(&sig, &key, &body)?;
    Ok(())
}