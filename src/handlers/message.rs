use crate::State;
use crate::{db, models::Message};
use tide::{convert::json, Response, StatusCode};

#[derive(serde::Deserialize)]
struct Query {
    room: String,
    count: Option<i64>,
}

/// Receive the last 20 chat messages
pub async fn list_messages(req: tide::Request<State>) -> tide::Result {
    let query: Query = req.query()?;

    let count = query.count.unwrap_or(20).clamp(1, 20);

    let messages: Vec<Message> = db::message::list(&req.state().pool, &query.room, count).await?;
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(json!(messages));
    Ok(res)
}
