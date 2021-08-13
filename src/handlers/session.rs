use http_types::StatusCode;
use serde_json::json;
use tide::Response;

use crate::{errors::TapseError, models, State};

#[derive(serde::Deserialize)]
struct SessionQuery {
    username: String,
    password: Option<String>,
}

pub async fn set_username(mut req: tide::Request<State>) -> tide::Result {
    let query: SessionQuery = req.body_json().await?;

    if let Some(set_pass) = &req.state().password {
        if query.password.is_none() {
            return Err(TapseError::WrongPassword.into());
        };

        if &query.password.unwrap() != set_pass {
            return Err(TapseError::WrongPassword.into());
        }
    }

    let session = req.session_mut();

    session.insert(
        "user",
        models::Session {
            username: query.username.clone(),
        },
    )?;

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(json!(&query.username));
    Ok(res)
}

pub async fn get_username(req: tide::Request<State>) -> tide::Result {
    let mut res = Response::new(StatusCode::Ok);

    let session: models::Session = match req.session().get("user") {
        Some(name) => name,
        None => {
            let res = Response::new(401);
            return Ok(res);
        }
    };

    res.set_body(json!(&session));
    Ok(res)
}
