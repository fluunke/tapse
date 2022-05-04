use crate::{
    db::{Room, RoomQuery},
    errors::TapseError,
    websocket::events::Response,
    ClientChannel, Database,
};
use axum::{http::StatusCode, Extension, Json};

use super::session::User;

pub async fn list_rooms(db: Extension<Database>, _: User) -> Result<Json<Vec<Room>>, TapseError> {
    let rooms: Vec<Room> = Room::list(&db).await?;

    Ok(Json(rooms))
}

pub async fn insert_room(
    room: Json<RoomQuery>,
    send: Extension<ClientChannel>,
    pool: Extension<Database>,
    _: User,
) -> Result<StatusCode, TapseError> {
    let new_room = Room::new(&pool, &room).await?;

    send.send(&Response::RoomCreated(new_room.clone()))
        .await
        .unwrap();

    Ok(StatusCode::OK)
}
