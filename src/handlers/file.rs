use crate::db::{self, File, FileQuery, RoomQuery};
use crate::errors::TapseError;
use crate::websocket::events::Response;
use crate::{ClientChannel, Database};
use axum::extract::{Multipart, Path, Query};
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::{Extension, Json};
use mime_guess::Mime;
use std::str::FromStr;

use super::session::User;

pub async fn upload_file(
    room: Query<RoomQuery>,
    broadcaster: Extension<ClientChannel>,
    pool: Extension<Database>,
    files: Multipart,
    _: User,
) -> Result<StatusCode, TapseError> {
    let files = db::File::insert(&pool, files, &room.room).await?;

    broadcaster.send(&Response::NewFiles(files)).await.unwrap();
    Ok(StatusCode::OK)
}

pub async fn list_files(
    room: Query<RoomQuery>,
    pool: Extension<Database>,
    _: User,
) -> Result<Json<Vec<File>>, TapseError> {
    let files = db::File::list(&pool, &room.room).await?;

    Ok(Json(files))
}

pub async fn view_file(
    Path(file): Path<FileQuery>,
    pool: Extension<Database>,
) -> Result<impl IntoResponse, TapseError> {
    let (file, bytes) = db::File::get(&pool, file).await?;

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
    Path(files): Path<Vec<FileQuery>>,
    pool: Extension<Database>,
    client: Extension<ClientChannel>,
    _: User,
) -> Result<StatusCode, TapseError> {
    db::File::delete(&pool, &files).await?;

    client.send(&Response::FilesDeleted(files)).await.unwrap();

    Ok(StatusCode::OK)
}
