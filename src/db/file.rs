use crate::{errors::TapseError, models::File};
use chrono::NaiveDateTime;
use multer::Multipart;
use sqlx::SqlitePool;

pub async fn list(room: &str, pool: &SqlitePool) -> Result<Vec<File>, TapseError> {
    match sqlx::query_as!(
        File,
        "select id, name, room, upload_date from files WHERE room = $1 ORDER BY upload_date DESC",
        room
    )
    .fetch_all(pool)
    .await
    {
        Ok(files) => Ok(files),
        Err(_) => Err(TapseError::NoFiles),
    }
}

pub async fn insert(
    mut payload: Multipart<'_>,
    room: &str,
    pool: &SqlitePool,
) -> Result<Vec<File>, TapseError> {
    let mut new_files: Vec<File> = Vec::new();

    while let Some(field) = payload.next_field().await.unwrap() {
        // Randomly generated ID for the file.
        let id = nanoid::nanoid!(7);

        let filename = match field.file_name() {
            Some(a) => urlencoding::encode(a).into_owned(),
            None => return Err(TapseError::FileName),
        };

        let bytes = field.bytes().await.unwrap().to_vec();

        // Disallow empty files
        if bytes.is_empty() {
            return Err(TapseError::FileEmpty);
        }

        new_files.push(sqlx::query_as!(
            File,
            r#"insert into files (id, name, room, file, upload_date) values ($1, $2, $3, $4, datetime('now')) returning id as "id!: String", name as "name!: String", upload_date as "upload_date!: NaiveDateTime", room as "room!: i64""#,
            id,
            filename,
            room,
            bytes
        )
        .fetch_one(pool)
        .await?);
    }

    Ok(new_files)
}

pub async fn get(
    file_id: &str,
    file_name: &str,
    pool: &SqlitePool,
) -> Result<(File, Vec<u8>), TapseError> {
    let q = sqlx::query!(
        r#"select * from files where id = $1 and name = $2"#,
        file_id,
        file_name
    )
    .fetch_one(pool)
    .await?;

    let file = File {
        id: q.id,
        name: q.name,
        room: q.room,
        upload_date: q.upload_date,
    };

    Ok((file, q.file))
}

pub(crate) async fn delete(
    file_id: &str,
    file_name: &str,
    pool: &SqlitePool,
) -> Result<String, TapseError> {
    sqlx::query!(
        r#"delete from files where id = $1 and name = $2"#,
        file_id,
        file_name
    )
    .execute(pool)
    .await?;

    Ok(String::from(file_id))
}
