use crate::websocket::events::{Request, Response};
use crate::{handlers::session::User, ClientChannel, Database};
use axum::{
    extract::{
        ws::{self, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    Extension,
};
use futures::{SinkExt, StreamExt};
use serde_json::json;

pub mod events;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user: User,
    Extension(pool): Extension<Database>,
    Extension(client): Extension<ClientChannel>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, user, pool, client))
}

async fn handle_socket(stream: WebSocket, user: User, pool: Database, client: ClientChannel) {
    let (mut sender, mut receiver) = stream.split();

    // Moving the user into closures angers the compiler
    let user_id = user.id.clone();

    let mut c = client.clone();

    // Messages from our server, sent to the client
    let mut recv_task = tokio::spawn(async move {
        while let Some(event) = c.next().await {
            let item = match event {
                Response::Error(ref e) => {
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

    let c = client.clone();

    // Messages coming from the client
    let mut send_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                ws::Message::Text(t) => {
                    let ev = Request::execute(&t, &user, &pool).await;
                    c.send(&ev).await.unwrap();
                }

                ws::Message::Close(_) => {
                    return;
                }
                _ => {}
            }
        }
    });

    // If one of the tasks panics, we need to close the other one
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
