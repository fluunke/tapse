use crate::db::Message;
use crate::errors::TapseError;
use crate::Database;
use axum::extract::Query;
use axum::{Extension, Json};
use serde::Deserialize;

use super::session::User;

#[derive(Deserialize)]
pub struct ListMessages {
    pub room: String,
    pub count: Option<i64>,
}

/// Receive the last 20 chat messages
pub async fn list_messages(
    q: Query<ListMessages>,
    pool: Extension<Database>,
    _: User,
) -> Result<Json<Vec<Message>>, TapseError> {
    let count = q.count.unwrap_or(20).clamp(1, 20);

    let messages: Vec<Message> = Message::list(&pool, &q.room, count).await?;
    Ok(Json(messages))
}
