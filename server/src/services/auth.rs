use super::{
    models::{Following, SignInRequest, SignUpRequest, User},
    service_utils::{create_jwt, get_db_connection, validate_key, MyError},
};
use ed25519_dalek::{PublicKey, SecretKey};
use mongodb::bson::doc;

pub async fn sign_up(user: SignInRequest) -> Result<(), MyError> {
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

pub async fn assign_keys_to_user(user: SignUpRequest) -> Result<(), MyError> {
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
    let user = db
        .collection::<User>("users")
        .find_one(doc! {"email": user.email}, None)
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
    let new_hash = bcrypt::hash_with_salt(&user.password, 4, new_salt);
    if new_hash.unwrap().format_for_version(bcrypt::Version::TwoB) != user.password {
        return Err(MyError {
            message: "Invalid password".to_string(),
            status: 401,
        });
    }

    db.collection::<User>("users")
        .update_one(
            doc! {"email": user.email},
            doc! {"$set": {"public_key": public_key_hex, "private_key": user.private_key}},
            None,
        )
        .await
        .map_err(|_| MyError {
            message: "Failed to update user".to_string(),
            status: 500,
        })?;

    Ok(())
}

pub async fn remove_user_keys(user: SignInRequest) -> Result<(), MyError> {
    let db = get_db_connection().await?;
    let user = db
        .collection::<User>("users")
        .find_one(doc! {"email": user.email}, None)
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
    let new_hash = bcrypt::hash_with_salt(&user.password, 4, new_salt);
    if new_hash.unwrap().format_for_version(bcrypt::Version::TwoB) != user.password {
        return Err(MyError {
            message: "Invalid password".to_string(),
            status: 401,
        });
    }

    db.collection::<User>("users")
        .update_one(
            doc! {"email": user.email},
            doc! {"$set": {"public_key": Option::<String>::None, "private_key": Option::<String>::None}},
            None,
        )
        .await
        .map_err(|_| MyError {
            message: "Failed to update user".to_string(),
            status: 500,
        })?;

    Ok(())
}

pub async fn follow_key(username: String, key: String) -> Result<(), MyError> {
    let db = get_db_connection().await?;
    let user = db
        .collection::<User>("users")
        .find_one(doc! {"email": &username  }, None)
        .await
        .map_err(|_| MyError {
            message: "Failed to get user".to_string(),
            status: 500,
        })?
        .ok_or(MyError {
            message: "User not found".to_string(),
            status: 404,
        })?;

    let following = db
        .collection::<Following>("following")
        .find_one(doc! {"user": user.email}, None)
        .await
        .map_err(|_| MyError {
            message: "Failed to get following".to_string(),
            status: 500,
        })?;

    match following {
        Some(_) => {
            db.collection::<Following>("following")
                .update_one(
                    doc! {"user": &username},
                    doc! {"$push": {"keys": key}},
                    None,
                )
                .await
                .map_err(|_| MyError {
                    message: "Failed to update following".to_string(),
                    status: 500,
                })?;
        }
        None => {
            let following = Following {
                user: username,
                keys: vec![key],
            };
            db.collection::<Following>("following")
                .insert_one(following, None)
                .await
                .map_err(|_| MyError {
                    message: "Failed to insert following".to_string(),
                    status: 500,
                })?;
        }
    }
    Ok(())
}

pub async fn get_followed_keys(username: String) -> Result<Vec<String>, MyError> {
    let db = get_db_connection().await?;
    let following = db
        .collection::<Following>("following")
        .find_one(doc! { "user": username }, None)
        .await
        .map_err(|_| MyError {
            message: "Failed to get following".to_string(),
            status: 500,
        })?;
    match following {
        Some(following) => Ok(following.keys),
        None => return Ok(vec![]),
    }
}

pub async fn unfollow_key(username: String, key: String) -> Result<(), MyError> {
    let db = get_db_connection().await?;
    let user = db
        .collection::<User>("users")
        .find_one(doc! {"email": &username  }, None)
        .await
        .map_err(|_| MyError {
            message: "Failed to get user".to_string(),
            status: 500,
        })?
        .ok_or(MyError {
            message: "User not found".to_string(),
            status: 404,
        })?;

    let following = db
        .collection::<Following>("following")
        .find_one(doc! {"user": user.email}, None)
        .await
        .map_err(|_| MyError {
            message: "Failed to get following".to_string(),
            status: 500,
        })?;

    match following {
        Some(_) => {
            db.collection::<Following>("following")
                .update_one(
                    doc! {"user": &username},
                    doc! {"$pull": {"keys": key}},
                    None,
                )
                .await
                .map_err(|_| MyError {
                    message: "Failed to update following".to_string(),
                    status: 500,
                })?;
        }
        None => {
            let following = Following {
                user: username,
                keys: vec![key],
            };
            db.collection::<Following>("following")
                .insert_one(following, None)
                .await
                .map_err(|_| MyError {
                    message: "Failed to insert following".to_string(),
                    status: 500,
                })?;
        }
    }
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
    let jwt = create_jwt(&user.email)?;
    Ok(jwt)
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
