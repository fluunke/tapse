use crate::errors::TapseError;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct Message {
    pub id: i64,
    pub author: String,
    pub room: String,
    pub content: String,
    pub creation_date: NaiveDateTime,
}

impl Message {
    pub async fn add(
        pool: &SqlitePool,
        author: &str,
        message: &MessageQuery,
    ) -> Result<Self, TapseError> {
        message.valid()?;

        Ok(sqlx::query_as!(
            Message,
            r#"
            insert into messages(room, author, content, creation_date)
            values ($1, $2, $3, datetime('now'))
            returning id, content, creation_date, room, author"#,
            message.room,
            author,
            message.content
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn list(pool: &SqlitePool, room: &str, amount: i64) -> Result<Vec<Self>, TapseError> {
        match sqlx::query_as!(
            Message,
            "
            select * from (select id, author, room, content, creation_date from messages
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
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MessageQuery {
    pub room: String,
    pub content: String,
}

impl MessageQuery {
    pub fn valid(&self) -> Result<(), TapseError> {
        let content = self.content.trim();

        if content.is_empty() || content.len() > 1024 {
            return Err(TapseError::MessageLength);
        }

        Ok(())
    }
}
