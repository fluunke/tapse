use std::str::FromStr;

use crate::websocket::WSEvent;
use crate::State;
use crate::{db, models::File};
use futures::FutureExt;
use http_types::Mime;
use multer::Multipart;
use tide::{convert::json, Request, Response, StatusCode};

#[derive(serde::Deserialize)]
struct FileRoom {
    room: String,
}

pub async fn upload_file(mut req: Request<State>) -> tide::Result {
    let room: FileRoom = req.query()?;

    let content_type = req.header("Content-Type").unwrap().get(0).unwrap().as_str();

    if !content_type.contains("multipart/form-data;") {
        return Ok(tide::Response::new(StatusCode::BadRequest));
    }

    let boundary_key = "boundary=";

    let skip_last_index = content_type.find(boundary_key).unwrap() + boundary_key.len();
    let boundary: String = content_type
        .chars()
        .skip(skip_last_index)
        .take(content_type.len() - skip_last_index)
        .collect();

    let stream = req.take_body().into_bytes().into_stream();
    let multipart = Multipart::new(stream, boundary);

    let files = db::file::insert(multipart, &room.room, &req.state().pool).await?;

    let send = req.state().broadcaster.clone();
    send.send(&WSEvent::NewFiles(files)).await?;

    Ok("uploaded".into())
}

pub async fn list_files(req: tide::Request<State>) -> tide::Result {
    let room: FileRoom = req.query()?;

    let files = db::file::list(&room.room, &req.state().pool).await?;
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(json!(files));
    Ok(res)
}

pub async fn view_file(req: tide::Request<State>) -> tide::Result {
    let file_id = req.param("id")?;
    let file_name = req.param("name")?;

    let file = db::file::get(file_id, file_name, &req.state().pool).await?;

    let mut res = Response::new(StatusCode::Ok);
    let mime = get_mime(&file.0);

    // Display/download files properly
    res.insert_header(
        "Content-Disposition",
        format!(
            "{}; filename=\"{}\"",
            should_download(mime.as_ref()),
            file.0.name
        ),
    );

    if let Some(mime) = mime {
        res.set_content_type(mime);
    }

    // Return the file binary
    res.set_body(file.1);
    Ok(res)
}

fn get_mime(f: &File) -> Option<Mime> {
    let filetype = match mime_guess::from_path(&f.name).first() {
        Some(m) => m,
        None => return None,
    }
    .to_string();

    Some(Mime::from_str(&filetype).unwrap())
}

fn should_download<'a>(mime: Option<&Mime>) -> &'a str {
    if let Some(mime) = mime {
        match mime.basetype() {
            // We want to display these in the browser
            "image" | "audio" | "text" | "video" => "inline",
            _ => "attachment",
        }
    } else {
        "attachment"
    }
}

pub async fn delete_file(req: tide::Request<State>) -> tide::Result {
    let file_id = req.param("id")?;
    let file_name = req.param("name")?;

    db::file::delete(file_id, file_name, &req.state().pool).await?;

    Ok(Response::new(200))
}
