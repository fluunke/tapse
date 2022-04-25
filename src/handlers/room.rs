use crate::{db::Room, errors::TapseError, Broadcaster, Database};
use axum::{http::StatusCode, Extension, Json};
use serde::{Deserialize, Serialize};

use crate::websocket::WSEvent;

pub async fn list_rooms(db: Extension<Database>) -> Result<Json<Vec<Room>>, TapseError> {
    let rooms: Vec<Room> = Room::list(&db).await?;

    Ok(Json(rooms))
}

#[derive(Deserialize, Serialize)]
pub struct InsertRoom {
    pub room: String,
}

pub async fn insert_room(
    room: Json<InsertRoom>,
    send: Extension<Broadcaster>,
    pool: Extension<Database>,
) -> Result<StatusCode, TapseError> {
    let new_room = Room::add(&pool, &room.room).await?;

    send.send(&WSEvent::NewRoom(new_room.clone()))
        .await
        .unwrap();

    Ok(StatusCode::OK)
}
