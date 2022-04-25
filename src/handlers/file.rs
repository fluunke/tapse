use crate::db::{self, File, FileQuery};
use crate::errors::TapseError;
use crate::websocket::WSEvent;
use crate::{Broadcaster, Database};
use axum::extract::{Multipart, Path, Query};
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::{Extension, Json};
use mime_guess::Mime;
use std::str::FromStr;

#[derive(serde::Deserialize)]
pub struct FileRoom {
    pub room: String,
}

pub async fn upload_file(
    room: Query<FileRoom>,
    broadcaster: Extension<Broadcaster>,
    pool: Extension<Database>,
    files: Multipart,
) -> Result<StatusCode, TapseError> {
    let files = db::File::insert(files, &room.room, &pool).await?;

    broadcaster.send(&WSEvent::NewFiles(files)).await.unwrap();
    Ok(StatusCode::OK)
}

pub async fn list_files(
    room: Query<FileRoom>,
    pool: Extension<Database>,
) -> Result<Json<Vec<File>>, TapseError> {
    let files = db::File::list(&room.room, &pool).await?;

    Ok(Json(files))
}

pub async fn view_file(
    Path(file): Path<FileQuery>,
    pool: Extension<Database>,
) -> Result<impl IntoResponse, TapseError> {
    let (file, bytes) = db::File::get(file, &pool).await?;

    let mut headers = HeaderMap::new();

    if let Some(mime) = get_mime(&file) {
        headers.insert(CONTENT_TYPE, mime.to_string().parse().unwrap());
    }

    Ok((headers, bytes))
}

fn get_mime(f: &File) -> Option<Mime> {
    let filetype = match mime_guess::from_path(&f.name).first() {
        Some(m) => m,
        None => return None,
    }
    .to_string();

    Some(Mime::from_str(&filetype).unwrap())
}

pub async fn delete_file(
    file: Path<FileQuery>,
    pool: Extension<Database>,
    broadcaster: Extension<Broadcaster>,
) -> Result<StatusCode, TapseError> {
    db::File::delete(&file, &pool).await?;

    broadcaster
        .send(&WSEvent::DeleteFile(file.id.clone()))
        .await
        .unwrap();

    Ok(StatusCode::OK)
}
