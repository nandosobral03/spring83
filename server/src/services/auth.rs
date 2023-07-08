use ed25519_dalek::{PublicKey, SecretKey};
use mongodb::bson::doc;
use rand::RngCore;
use serde::{Deserialize, Serialize};

use super::service_utils::{get_db_connection, validate_key, MyError};
use crate::routes::auth_routes::{SignInRequest, SignUpRequest};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub email: String,
    pub password: String,
    pub salt: String,
    pub public_key: String,
    pub private_key: String,
}

impl From<SignUpRequest> for User {
    fn from(user: SignUpRequest) -> Self {
        let private_key: SecretKey =
            SecretKey::from_bytes(&hex::decode(&user.private_key).unwrap()).unwrap();
        let public_key: PublicKey = PublicKey::from(&private_key);
        let public_key_hex: String = hex::encode(public_key.to_bytes());
        let mut salt = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut salt);
        let password_hash = bcrypt::hash_with_salt(user.password, 4, salt).unwrap();
        User {
            email: user.email,
            password: password_hash.format_for_version(bcrypt::Version::TwoB),
            private_key: user.private_key,
            public_key: public_key_hex,
            salt: hex::encode(salt),
        }
    }
}

pub async fn sign_up(user: SignUpRequest) -> Result<(), MyError> {
    let private_key: SecretKey =
        SecretKey::from_bytes(&hex::decode(&user.private_key).map_err(|_| MyError {
            message: "Invalid private key".to_string(),
            status: 400,
        })?)
        .map_err(|_| MyError {
            message: "Invalid private key".to_string(),
            status: 400,
        })?;
    let public_key: PublicKey = PublicKey::from(&private_key);
    let public_key_hex: String = hex::encode(public_key.to_bytes());
    validate_key(&public_key_hex)?;

    let db = get_db_connection().await?;
    let user_exists = db
        .collection::<User>("users")
        .find_one(doc! {"email": &user.email}, None)
        .await
        .map_err(|_| MyError {
            message: "Failed to get user".to_string(),
            status: 500,
        })?;
    if user_exists.is_some() {
        return Err(MyError {
            message: "User already exists".to_string(),
            status: 409,
        });
    }

    let user_doc = User::from(user);
    db.collection::<User>("users")
        .insert_one(user_doc, None)
        .await
        .map_err(|_| MyError {
            message: "Failed to insert user".to_string(),
            status: 500,
        })?;
    Ok(())
}

pub async fn sign_in(user_login: SignInRequest) -> Result<String, MyError> {
    let db = get_db_connection().await?;
    let user = db
        .collection::<User>("users")
        .find_one(doc! {"email": user_login.email}, None)
        .await
        .map_err(|_| MyError {
            message: "Failed to get user".to_string(),
            status: 500,
        })?
        .ok_or(MyError {
            message: "User not found".to_string(),
            status: 404,
        })?;

    let mut new_salt = [0u8; 16];
    new_salt.copy_from_slice(&hex::decode(user.salt).unwrap());
    let new_hash = bcrypt::hash_with_salt(&user_login.password, 4, new_salt);
    if new_hash.unwrap().format_for_version(bcrypt::Version::TwoB) != user.password {
        return Err(MyError {
            message: "Invalid password".to_string(),
            status: 400,
        });
    }

    Ok(format!("{}{}", user.private_key, user.public_key))
}

pub async fn remove_user(user_login: SignInRequest) -> Result<(), MyError> {
    let db = get_db_connection().await?;
    let user = db
        .collection::<User>("users")
        .find_one(doc! {"email": user_login.email}, None)
        .await
        .map_err(|_| MyError {
            message: "Failed to get user".to_string(),
            status: 500,
        })?
        .ok_or(MyError {
            message: "User not found".to_string(),
            status: 404,
        })?;

    let mut new_salt = [0u8; 16];
    new_salt.copy_from_slice(&hex::decode(user.salt).unwrap());
    let new_hash = bcrypt::hash_with_salt(&user_login.password, 4, new_salt);
    if new_hash.unwrap().format_for_version(bcrypt::Version::TwoB) != user.password {
        return Err(MyError {
            message: "Invalid password".to_string(),
            status: 400,
        });
    }

    db.collection::<User>("users")
        .delete_one(doc! {"email": user.email}, None)
        .await
        .map_err(|_| MyError {
            message: "Failed to delete user".to_string(),
            status: 500,
        })?;
    Ok(())
}
