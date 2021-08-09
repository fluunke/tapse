pub mod file;
pub mod message;
pub mod room;
pub mod session;

use std::borrow::Cow;

use crate::{Frontend, State};
use http_types::Body;
use mime_guess::from_path;
use tide::{Response, StatusCode};

pub async fn handle_embedded_file(req: tide::Request<State>) -> tide::Result {
    let mut res = Response::new(StatusCode::Ok);

    // If no url parameter is given (e.g. 'localhost:8080/'),
    // we can safely assume that we want index.html
    let path = req.param("path").unwrap_or("index.html");

    match Frontend::get(path) {
        Some(content) => {
            let body: Body = match content.data {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };
            res.set_content_type(from_path(&path).first_or_octet_stream().as_ref());
            res.set_body(body);
            Ok(res)
        }
        None => {
            res.set_status(StatusCode::NotFound);
            Ok(res)
        }
    }
}
