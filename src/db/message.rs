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
        r#"insert into messages(room, author, content, creation_date)
        values ($1, $2, $3, datetime('now'))
        returning id as "id!: i64", content as "content!: String", creation_date as "creation_date!: NaiveDateTime", room as "room!: i64", author as "author!: String""#,
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
        "select * from (select id, author, room, content, creation_date from messages
         where room = $1
         order by creation_date desc limit $2)
         as x order by creation_date asc",
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
