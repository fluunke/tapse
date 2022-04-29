use crate::errors::TapseError;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Debug)]
pub struct Room {
    pub id: i64,
    pub name: String,
    pub creation_date: NaiveDateTime,
}

impl Room {
    pub async fn add(pool: &SqlitePool, room: &str) -> Result<Room, TapseError> {
        let room = room.trim();

        // Handle too long/short room names
        match room.len() {
            n if n >= 20 => return Err(TapseError::RoomNameTooLong),
            n if n <= 3 => return Err(TapseError::RoomNameTooShort),
            _ => {}
        }

        match sqlx::query_as!(
            Room,
            r#"
            insert into rooms (name, creation_date)
            values ($1, datetime('now')) returning id, name, creation_date
        "#,
            room
        )
        .fetch_one(pool)
        .await
        {
            Ok(room) => Ok(room),
            Err(e) => Err(TapseError::RoomCreationError(e)),
        }
    }

    pub async fn list(pool: &SqlitePool) -> Result<Vec<Room>, TapseError> {
        Ok(sqlx::query_as!(
            Room,
            r#"
            select id, name, creation_date from rooms"#
        )
        .fetch_all(pool)
        .await?)
    }
}
