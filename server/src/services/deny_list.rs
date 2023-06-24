use mongodb::bson::doc;
use serde::{Serialize, Deserialize};
use super::service_utils::{MyError, get_db_connection};

pub async fn add_denied_key(key : String) -> Result<(), MyError> {
    let client = get_db_connection().await?;
    println!("Adding key: {}", &key);
    if !is_denied_key(&key).await? {
        println!("Key not denied");
        client.collection("deny_list").insert_one(doc! {"key": key}, None).await.map_err(|e| MyError{message: e.to_string(), status : 500})?;
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct DeniedKey {
    key : String
}

pub async fn is_denied_key(key : &String) -> Result<bool, MyError> {
    let client = get_db_connection().await?;
    let filter = doc! { "key": key };
    
    let denied = client
    .collection::<DeniedKey>("deny_list")
    .find_one(filter, None)
    .await
    .map_err(|e| MyError {
        message: format!("Failed to get board: {}", e),
        status: 500,
    })?;
    Ok(denied.is_some())
}

pub async fn delete_denied_key(key : String) -> Result<(), MyError> {
    let client = get_db_connection().await?;
    let filter = doc! { "key": key };
    
    client
    .collection::<String>("deny_list")
    .delete_one(filter, None)
    .await
    .map_err(|e| MyError {
        message: format!("Failed to get board: {}", e),
        status: 500,
    })?;
    Ok(())
}


pub async fn get_denied_keys() -> Result<Vec<String>, MyError> {
    let client = get_db_connection().await?;
    let filter = doc! {};
    
    let mut denied = client
    .collection::<String>("deny_list")
    .find(filter, None)
    .await
    .map_err(|e| MyError {
        message: format!("Failed to get board: {}", e),
        status: 500,
    })?;
    let mut keys = Vec::new();
    while denied.advance().await.map_err(|e| MyError {
        message: format!("Failed to get board: {}", e),
        status: 500,
    })? {
        let doc = denied.current();
        let key = doc.get_str("key").unwrap();
        keys.push(key.to_string());
    }
    Ok(keys)
}
