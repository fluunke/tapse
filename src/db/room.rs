use crate::{errors::TapseError, models::Room};
use chrono::NaiveDateTime;
use sqlx::SqlitePool;

pub async fn add(pool: &SqlitePool, room: String) -> Result<Room, TapseError> {
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
        values ($1, datetime('now')) returning id as "id!: i64", name as "name!: String", creation_date as "creation_date!: NaiveDateTime"
    "#, room
    )
    .fetch_one(pool)
    .await{
        Ok(room) => Ok(room),
        Err(e) => Err(TapseError::RoomCreationError(e))
    }
}

pub async fn list(pool: &SqlitePool) -> Result<Vec<Room>, TapseError> {
    let rooms = sqlx::query_as!(
        Room,
        r#"
        select id, name, creation_date from rooms"#
    )
    .fetch_all(pool)
    .await?;

    Ok(rooms)
}
