use ed25519_dalek::{PublicKey, SecretKey};
use mongodb::bson::doc;
use rand::RngCore;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    pub private_key: String,
}

#[derive(Deserialize)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub email: String,
    pub password: String,
    pub salt: String,
    pub public_key: Option<String>,
    pub private_key: Option<String>,
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
            private_key: Some(user.private_key),
            public_key: Some(public_key_hex),
            salt: hex::encode(salt),
        }
    }
}

impl From<SignInRequest> for User {
    fn from(user: SignInRequest) -> Self {
        let mut salt = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut salt);
        let password_hash = bcrypt::hash_with_salt(user.password, 4, salt).unwrap();
        User {
            email: user.email,
            password: password_hash.format_for_version(bcrypt::Version::TwoB),
            private_key: None,
            public_key: None,
            salt: hex::encode(salt),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Following {
    pub user: String,
    pub keys: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeypairDisplay {
    pub public_key: String,
    pub private_key: String,
}
