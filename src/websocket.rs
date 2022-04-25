use crate::db::{File, FileQuery, Message, MessageQuery, Room};
use crate::errors::TapseError;
use crate::{handlers::session::User, Broadcaster, Database};
use axum::{
    extract::{
        ws::{self, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    Extension,
};
use futures::{SinkExt, StreamExt};
use serde_json::{from_str, json};
use sqlx::{Pool, Sqlite};
use tracing::{debug, warn};

#[derive(Clone, serde::Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum WSEvent {
    NewMessage(Message),
    NewRoom(Room),
    NewFiles(Vec<File>),
    DeleteFile(String),
    Error(WSError),
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct WSError {
    #[serde(skip_serializing)]
    pub recipient_id: String,
    pub message: String,
}

impl WSEvent {
    pub fn error(recipient_id: &str, message: TapseError) -> Self {
        WSEvent::Error(WSError {
            recipient_id: recipient_id.to_string(),
            message: message.to_string(),
        })
    }
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user: User,
    Extension(pool): Extension<Database>,
    Extension(broadcaster): Extension<Broadcaster>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, user, pool, broadcaster))
}

async fn handle_socket(stream: WebSocket, user: User, pool: Database, broadcaster: Broadcaster) {
    let (mut sender, mut receiver) = stream.split();

    let mut b = broadcaster.clone();

    // Moving the user into closures angers the compiler
    let user_id = user.id.clone();

    let mut recv_task = tokio::spawn(async move {
        while let Some(event) = b.next().await {
            let item = match event {
                WSEvent::Error(ref e) => {
                    if e.recipient_id == user_id {
                        ws::Message::Text(json!(event).to_string())
                    } else {
                        continue;
                    }
                }
                _ => ws::Message::Text(json!(event).to_string()),
            };

            sender.send(item).await.unwrap();
        }
    });

    let mut send_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                ws::Message::Text(t) => {
                    warn!("client send str: {:?}", t);

                    let action = match_ws_action(&t, &user, &pool).await;
                    broadcaster.send(&action).await.unwrap();
                }

                ws::Message::Close(_) => {
                    warn!("client disconnected");
                    return;
                }
                _ => {}
            }
        }
    });
    warn!("running tasks");

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}

async fn match_ws_action(input: &str, user: &User, pool: &Pool<Sqlite>) -> WSEvent {
    // Handle new message
    if let Ok(msg) = from_str::<MessageQuery>(input) {
        debug!("New message: {:?}", &msg);

        match Message::add(pool, &user.username, &msg).await {
            Ok(new) => WSEvent::NewMessage(new),
            Err(e) => WSEvent::error(&user.id, e),
        }
    }
    // Handle file being deleted
    else if let Ok(file) = from_str::<FileQuery>(input) {
        debug!("Deleting file: {:?}", &file);

        match File::delete(&file, pool).await {
            Ok(_) => WSEvent::DeleteFile(file.id),
            Err(e) => WSEvent::error(&user.id, e),
        }
    }
    // Handle invalid queries
    else {
        debug!("Invalid websocket query received: {}", input);
        WSEvent::error(&user.id, TapseError::InvalidQuery)
    }
}
