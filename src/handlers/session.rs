use http_types::StatusCode;
use models::Session;
use serde_json::json;
use tide::Response;

use crate::{models, State};

pub async fn set_username(mut req: tide::Request<State>) -> tide::Result {
    let user: models::Session = req.body_json().await?;

    let session = req.session_mut();

    session.insert("user", &user)?;

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(json!(&user));
    Ok(res)
}

pub async fn get_username(req: tide::Request<State>) -> tide::Result {
    let mut res = Response::new(StatusCode::Ok);
    let session: Session = match req.session().get("user") {
        Some(name) => name,
        None => {
            let mut res = Response::new(200);
            res.set_body("No username set");
            return Ok(res);
        }
    };

    res.set_body(json!(&session));
    Ok(res)
}
