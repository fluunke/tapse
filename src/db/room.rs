use crate::errors::TapseError;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct Room {
    pub name: String,
    pub creation_date: NaiveDateTime,
}

impl Room {
    pub async fn new(pool: &SqlitePool, room: &RoomQuery) -> Result<Self, TapseError> {
        room.valid()?;

        Ok(sqlx::query_as!(
            Room,
            r#"
            insert into rooms (name, creation_date)
            values ($1, datetime('now')) returning name, creation_date
        "#,
            room.room
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn list(pool: &SqlitePool) -> Result<Vec<Self>, TapseError> {
        Ok(sqlx::query_as!(
            Room,
            r#"
            select name, creation_date from rooms"#
        )
        .fetch_all(pool)
        .await?)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RoomQuery {
    pub room: String,
}

impl RoomQuery {
    pub fn valid(&self) -> Result<(), TapseError> {
        let len = self.room.len();

        if !(3..20).contains(&len) {
            return Err(TapseError::RoomNameLength);
        };
        Ok(())
    }
}
