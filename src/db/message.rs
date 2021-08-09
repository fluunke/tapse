use crate::{errors::TapseError, models::Message, models::MessageIn};
use chrono::NaiveDateTime;
use sqlx::SqlitePool;

pub async fn add(
    pool: &SqlitePool,
    author: &str,
    message: &MessageIn,
) -> Result<Message, TapseError> {
    // Reject empty messages...
    if message.content.trim().is_empty() {
        return Err(TapseError::MessageTooShort);
    }
    // ...and too long messages
    else if message.content.trim().len() > 1000 {
        return Err(TapseError::MessageTooLong);
    };

    Ok(sqlx::query_as!(
            Message,
        r#"INSERT INTO messages(room, author, content, creation_date)
        VALUES ($1, $2, $3, datetime('now'))
        RETURNING id as "id!: i64", content as "content!: String", creation_date as "creation_date!: NaiveDateTime", room as "room!: i64", author as "author!: String""#,
            message.room,
            author,
            message.content
        )
        .fetch_one(pool).await?
        )
}

pub async fn list(pool: &SqlitePool, room: &str, amount: i64) -> Result<Vec<Message>, TapseError> {
    match sqlx::query_as!(
        Message,
        "SELECT * FROM (SELECT id, author, room, content, creation_date FROM messages
         WHERE room = $1
         ORDER BY creation_date DESC LIMIT $2)
         AS x ORDER BY creation_date ASC",
        room,
        amount
    )
    .fetch_all(pool)
    .await
    {
        Ok(messages) => Ok(messages),
        Err(_) => Err(TapseError::NoMessages),
    }
}
