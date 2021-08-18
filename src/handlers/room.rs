use crate::{db, models::Room};
use serde::{Deserialize, Serialize};

use crate::{websocket::WSEvent, State};
use tide::{convert::json, Response, StatusCode};

pub async fn list_rooms(req: tide::Request<State>) -> tide::Result {
    let rooms: Vec<Room> = db::room::list(&req.state().pool).await?;
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(json!(rooms));
    Ok(res)
}

#[derive(Deserialize, Serialize)]
pub struct InsertRoom {
    pub room: String,
}

pub async fn insert_room(mut req: tide::Request<State>) -> tide::Result {
    let room: InsertRoom = req.body_json().await?;
    let send = req.state().broadcaster.clone();

    let new_room = db::room::add(&req.state().pool, room.room).await?;

    send.send(&WSEvent::NewRoom(new_room.clone())).await?;

    let mut res = Response::new(200);
    res.set_body(json!(new_room));
    Ok(res)
}
