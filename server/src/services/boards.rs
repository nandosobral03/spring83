use crate::routes::boards_routes::Orientation;

use super::{
    deny_list::is_denied_key,
    models::Following,
    service_utils::{
        get_db_connection, validate_key, validate_signature, validate_timestamp, MyError,
    },
};
use chrono::{DateTime, TimeZone, Utc};
use ed25519_dalek::*;
use mongodb::{
    bson::{self, doc},
    options::FindOptions,
};
use serde::{Deserialize, Serialize};

pub async fn put_board(
    key: String,
    sig: String,
    orientation: Orientation,
    body: &String,
) -> Result<(), MyError> {
    validate_key(&key)?;
    let is_empty = board_is_empty(&body)?;
    let timestamp = validate_timestamp(&body)?;
    if is_denied_key(&key).await? {
        return Err(MyError {
            message: "Forbidden".to_string(),
            status: 403,
        });
    }
    validate_signature(&sig, &key, &body)?;
    if is_empty {
        remove_board(&key).await?;
    } else {
        save_board(&key, &body, &timestamp, orientation, &sig).await?;
    }
    Ok(())
}

pub async fn remove_board(key: &str) -> Result<(), MyError> {
    let client = get_db_connection().await?;
    client
        .collection::<Board>("boards")
        .delete_one(doc! { "_id": key }, None)
        .await
        .map_err(|_| MyError {
            message: "Failed to delete board".to_string(),
            status: 500,
        })?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Board {
    #[serde(rename = "_id")]
    pub key: String,
    pub body: String,
    pub timestamp: String,
    pub last_modified: i64,
    pub signature: String,
    pub orientation: Orientation,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BoardDisplay {
    pub body: String,
    pub timestamp: String,
    pub last_modified: String,
    pub signature: String,
    pub orientation: Orientation,
    pub key: String,
}

impl From<Board> for BoardDisplay {
    fn from(board: Board) -> Self {
        let last_modified = chrono::Utc
            .timestamp_millis_opt(board.last_modified)
            .unwrap()
            .to_rfc3339()
            .parse()
            .unwrap();
        BoardDisplay {
            body: board.body,
            timestamp: board.timestamp,
            last_modified,
            signature: board.signature,
            orientation: board.orientation,
            key: board.key,
        }
    }
}

async fn save_board(
    key: &str,
    body: &str,
    timestamp: &str,
    orientation: Orientation,
    sig: &str,
) -> Result<(), MyError> {
    let client = get_db_connection().await?;

    let board = Board {
        key: key.to_string(),
        body: body.to_string(),
        timestamp: timestamp.to_string(),
        last_modified: chrono::Utc::now().timestamp_millis(),
        signature: sig.to_string(),
        orientation,
    };

    let existsing_board = client
        .collection::<Board>("boards")
        .find_one(doc! { "_id": key }, None)
        .await
        .map_err(|_| MyError {
            message: "Failed to get board".to_string(),
            status: 500,
        })?;
    if let Some(board) = existsing_board {
        if chrono::DateTime::parse_from_rfc3339(&board.timestamp).unwrap()
            > chrono::DateTime::parse_from_rfc3339(timestamp).unwrap()
        {
            return Err(MyError {
                message: "Timestamp is older than existing board".to_string(),
                status: 409,
            });
        } else {
            client
                .collection::<Board>("boards")
                .delete_one(doc! { "_id": key }, None)
                .await
                .map_err(|_| MyError {
                    message: "Failed to delete board".to_string(),
                    status: 500,
                })?;
        }
    }

    client
        .collection::<Board>("boards")
        .insert_one(board, None)
        .await
        .map_err(|e| MyError {
            message: format!("Failed to save board: {}", e),
            status: 500,
        })?;
    Ok(())
}

pub async fn get_board(
    key: &str,
    modified_since: Option<DateTime<Utc>>,
) -> Result<BoardDisplay, MyError> {
    let client = get_db_connection().await?;
    let filter = doc! { "_id": key };
    let board = client
        .collection::<Board>("boards")
        .find_one(filter, None)
        .await
        .map_err(|e| MyError {
            message: format!("Failed to get board: {}", e),
            status: 500,
        })?;
    match board {
        Some(board) => {
            let last_modified: DateTime<Utc> =
                Utc.timestamp_millis_opt(board.last_modified).unwrap();
            if let Some(modified_since) = modified_since {
                if last_modified <= modified_since {
                    return Err(MyError {
                        message: "Not modified".to_string(),
                        status: 304,
                    });
                }
            }
            Ok(board.into())
        }
        None => {
            return Err(MyError {
                message: "Board not found".to_string(),
                status: 404,
            })
        }
    }
}

fn board_is_empty(document: &str) -> Result<bool, MyError> {
    let dom = tl::parse(document, tl::ParserOptions::default()).map_err(|_| MyError {
        message: "Failed to parse document".to_string(),
        status: 400,
    })?;
    let children = dom.nodes();
    return Ok(children.len() <= 1);
}

pub async fn get_test_board() -> Result<BoardDisplay, MyError> {
    let timestamp = chrono::Utc::now().to_rfc3339();
    #[derive(Deserialize)]
    struct RandomFact {
        text: String,
    }

    let random_fact = reqwest::get("https://uselessfacts.jsph.pl/random.json?language=en")
        .await
        .map_err(|_| MyError {
            message: "Failed to get random fact for test board".to_string(),
            status: 500,
        })?
        .json::<RandomFact>()
        .await
        .map_err(|_| MyError {
            message: "Failed to parse random fact for test board".to_string(),
            status: 500,
        })?;

    let random_color = format!(
        "rgb({},{},{})",
        rand::random::<u8>(),
        rand::random::<u8>(),
        rand::random::<u8>()
    );

    let body = format!(
        " \
    <time datetime=\"{}\"></time> \
    <div>
        <h1>This is a test board generated by the server</h1>
        <h3> At the time of generation, the server time was {} </h3>
        <p> Enjoy this random fact: </p>
        <p> {} </p>
        <div style=\"background-color:{}; width: 100px; height: 100px;\">

        </div>
    </div>
   ",
        timestamp, timestamp, random_fact.text, random_color
    );

    let public_key = "ab589f4dde9fce4180fcf42c7b05185b0a02a5d682e353fa39177995083e0583";
    let secret_key = "3371f8b011f51632fea33ed0a3688c26a45498205c6097c352bd4d079d224419";
    let keypair_string = format!("{}{}", secret_key, public_key);
    let keypair_bytes = hex::decode(keypair_string).unwrap();
    let keypair: Keypair = Keypair::from_bytes(&keypair_bytes).unwrap();
    let signature = keypair.sign(body.as_bytes());
    let signature = hex::encode(signature.to_bytes());

    let board: Board = Board {
        key: public_key.to_string(),
        body: body.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        last_modified: chrono::Utc::now().timestamp(),
        signature: signature.to_string(),
        orientation: Orientation::Portrait,
    };
    Ok(board.into())
}

pub async fn get_recently_updated_boards(
    limit: i64,
    skip: u64,
) -> Result<Vec<BoardDisplay>, MyError> {
    let client = get_db_connection().await?;
    let options = FindOptions::builder()
        .sort(doc! { "last_modified": -1 })
        .limit(limit)
        .skip(skip)
        .build();
    let mut boards = client
        .collection::<Board>("boards")
        .find(None, options)
        .await
        .map_err(|e| MyError {
            message: format!("Failed to get boards: {}", e),
            status: 500,
        })?;
    let mut boards_vec: Vec<BoardDisplay> = vec![];
    while boards.advance().await.map_err(|e| MyError {
        message: format!("Failed to get boards: {}", e),
        status: 500,
    })? {
        let board: Board = bson::from_bson(bson::Bson::Document(
            boards
                .current()
                .to_raw_document_buf()
                .to_document()
                .unwrap()
                .clone(),
        ))
        .unwrap();

        boards_vec.push(board.into());
    }
    Ok(boards_vec)
}

pub async fn get_followed_boards(username: String) -> Result<Vec<BoardDisplay>, MyError> {
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
        Some(following) => {
            let mut boards = vec![];
            for key in following.keys {
                let board = get_board(&key, None).await;
                if let Ok(board) = board {
                    boards.push(board);
                }
                boards.sort_unstable_by(|a, b| b.last_modified.cmp(&a.last_modified));
            }
            Ok(boards)
        }
        None => return Ok(vec![]),
    }
}

pub async fn get_boards_count() -> Result<u64, MyError> {
    let client = get_db_connection().await?;
    let count = client
        .collection::<Board>("boards")
        .count_documents(None, None)
        .await
        .map_err(|e| MyError {
            message: format!("Failed to get boards count: {}", e),
            status: 500,
        })?;
    Ok(count)
}
