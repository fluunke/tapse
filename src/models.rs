use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct Message {
    pub id: i64,
    pub author: String,
    pub room: i64,
    pub content: String,
    pub creation_date: NaiveDateTime,
}

/// Message struct used for user input
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MessageIn {
    pub room: i64,
    pub content: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DeleteFile {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Debug)]
pub struct Room {
    pub id: i64,
    pub name: String,
    pub creation_date: NaiveDateTime,
}

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct File {
    pub id: String,
    pub room: i64,
    pub name: String,
    pub upload_date: NaiveDateTime,
}

/// Wraps error messages into something json-serializable
#[derive(Serialize, Clone, Debug)]
pub struct WSError {
    pub error: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Session {
    pub username: String,
}

impl Default for Session {
    fn default() -> Self {
        Session {
            username: "Anonymous".to_string(),
        }
    }
}
