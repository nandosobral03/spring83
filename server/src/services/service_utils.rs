use std::env;

use chrono::Datelike;
use dotenvy::dotenv;
use ed25519_dalek::Verifier;
use mongodb::{options::ClientOptions, Client, Database};

pub struct MyError {
    pub message: String,
    pub status: u16,
}

pub fn validate_key(key: &str) -> Result<(), MyError> {
    // [a-zA-Z0-9]+83e(0[1-9]|1[0-2])(\d\d)$
    let re = regex::Regex::new(r"^[a-zA-Z0-9]+83e(0[1-9]|1[0-2])(\d\d)$").unwrap();
    let key_len = key.len();
    if !re.is_match(key) || key_len != 64 {
        return Err(MyError {
            message: "Forbidden".to_string(),
            status: 403,
        });
    }
    let month = &key[64 - 4..64 - 2];
    let year = &key[64 - 2..64];
    let month: u8 = month.parse().unwrap();
    let year: u8 = year.parse().unwrap();

    let current_year = ((chrono::Local::now().year()) % 100) as u8;
    let current_month = chrono::Local::now().month() as u8;

    if year < current_year - 2 {
        return Err(MyError {
            message: "Expired key".to_string(),
            status: 403,
        });
    }
    if year > current_year {
        return Err(MyError {
            message: "Key not valid yet".to_string(),
            status: 403,
        });
    }

    if year == current_year - 2 && month < current_month {
        return Err(MyError {
            message: "Expired key".to_string(),
            status: 403,
        });
    }

    if year == current_year && month > current_month {
        return Err(MyError {
            message: "Expired key".to_string(),
            status: 403,
        });
    }

    Ok(())
}

pub fn validate_timestamp(body: &String) -> Result<String, MyError> {
    let datetime = get_datetime(body)?;
    // ISO 8601 format YYYY-MM-DDTHH:MM:SSZ
    let time =
        chrono::NaiveDateTime::parse_from_str(&datetime, "%Y-%m-%dT%H:%M:%SZ").map_err(|_| {
            MyError {
                message: "Incorrect timestamp format".to_string(),
                status: 400,
            }
        })?;
    let current_time = chrono::Local::now().naive_local();
    if time > current_time {
        return Err(MyError {
            message: "Timestamp in the future".to_string(),
            status: 400,
        });
    }

    if time < current_time - chrono::Duration::days(22) {
        return Err(MyError {
            message: "Timestamp more than 22 days old".to_string(),
            status: 400,
        });
    }

    Ok(datetime)
}

fn get_datetime(body: &String) -> Result<String, MyError> {
    let dom = tl::parse(body, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();
    if let Some(mut element) = dom.query_selector("time[datetime]") {
        if let Some(timestamp) = element.next() {
            if let Some(node) = timestamp.get(parser) {
                if let Some(Some(timestamp)) = node.as_tag().unwrap().attributes().get("datetime") {
                    if let Some(timestamp) = timestamp.try_as_utf8_str() {
                        return Ok(timestamp.to_string());
                    } else {
                        return Err(MyError {
                            message: "Incorrect timestamp format".to_string(),
                            status: 400,
                        });
                    }
                }
            }
        }
    }
    return Err(MyError {
        message: "No timestamp found".to_string(),
        status: 400,
    });
}

pub fn validate_signature(sig: &String, key: &String, body: &String) -> Result<(), MyError> {
    let public_key = ed25519_dalek::PublicKey::from_bytes(&hex::decode(key).unwrap()).unwrap();
    let signature = ed25519_dalek::Signature::from_bytes(&hex::decode(sig).unwrap()).unwrap();

    public_key
        .verify(body.as_bytes(), &signature)
        .map_err(|_| MyError {
            message: "Invalid signature".to_string(),
            status: 403,
        })?;
    // TODO validate blacklist and denylist
    Ok(())
}

pub async fn get_db_connection() -> Result<Database, MyError> {
    dotenv().map_err(|_| MyError {
        message: "Failed to read .env file".to_string(),
        status: 500,
    })?;
    let database_url = env::var("DATABASE_URL").map_err(|_| MyError {
        message: "Failed to read DATABASE_URL from .env file".to_string(),
        status: 500,
    })?;
    let client_options = ClientOptions::parse(database_url)
        .await
        .map_err(|_| MyError {
            message: "Failed to parse database url".to_string(),
            status: 500,
        })?;
    let client = Client::with_options(client_options).map_err(|_| MyError {
        message: "Failed to connect to database".to_string(),
        status: 500,
    })?;
    Ok(client.database("spring83"))
}
