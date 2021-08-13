use crate::{
    db,
    models::{DeleteFile, File, Session},
};
use crate::{
    models::{Message, MessageIn, Room},
    State,
};
use async_std::task::JoinHandle;
use futures::StreamExt;
use serde_json::from_str;
use sqlx::{Pool, Sqlite};
use tide::{log::debug, Request};
use tide_websockets::WebSocket;

#[derive(Clone, serde::Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum WSEvent {
    NewMessage(Message),
    NewRoom(Room),
    NewFiles(Vec<File>),
    DeleteFile(String),
    Error(String),
}

pub fn make_ws(mut app: tide::Route<State>) -> tide::Route<State> {
    app.at("/ws")
        .get(WebSocket::new(|req: Request<State>, stream| async move {
            let session: Session = req.session().get("user").unwrap_or_default();

            let mut rec = req.state().broadcaster.clone();
            let ws_conn = stream.clone();

            let receiver: JoinHandle<tide::Result> = async_std::task::spawn(async move {
                loop {
                    let event = rec.next().await.unwrap();
                    ws_conn.send_json(&event).await?
                }
            });

            let mut send_stream = stream.clone();
            let sender: JoinHandle<tide::Result> = async_std::task::spawn(async move {
                let sender = req.state().broadcaster.clone();
                while let Some(Ok(tide_websockets::Message::Text(input))) = send_stream.next().await
                {
                    let pool = req.state().pool.clone();
                    let action = match_ws_action(&input, &session, &pool).await;
                    sender.send(&action).await?;
                }
                Ok("ok".into())
            });

            receiver.await?;
            sender.await?;

            Ok(())
        }));

    app
}

async fn match_ws_action(input: &str, session: &Session, pool: &Pool<Sqlite>) -> WSEvent {
    // Handle new message
    if let Ok(msg) = from_str::<MessageIn>(input) {
        debug!("New message: {:?}", &msg);

        match db::message::add(pool, &session.username, &msg).await {
            Ok(new) => WSEvent::NewMessage(new),
            Err(e) => WSEvent::Error(e.to_string()),
        }
    }
    // Handle file being deleted
    else if let Ok(file) = from_str::<DeleteFile>(input) {
        debug!("Deleting file: {:?}", &file);

        match db::file::delete(&file.id, &file.name, pool).await {
            Ok(deleted_file) => WSEvent::DeleteFile(deleted_file),
            Err(e) => WSEvent::Error(e.to_string()),
        }
    }
    // Handle invalid queries
    else {
        debug!("Invalid websocket query received: {}", input);
        WSEvent::Error("Invalid query".to_string())
    }
}
