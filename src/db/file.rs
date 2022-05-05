use crate::{errors::TapseError, websocket::events::FileMoveRequest};
use axum::extract::Multipart;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone, Debug)]
pub struct File {
    pub id: String,
    pub room: String,
    pub name: String,
    pub upload_date: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FileQuery {
    pub id: String,
    pub name: String,
}

impl File {
    pub async fn list(pool: &SqlitePool, room: &str) -> Result<Vec<Self>, TapseError> {
        match sqlx::query_as!(
            File,
            "select id, name, room, upload_date from files
             WHERE room = $1
             ORDER BY upload_date DESC",
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
        pool: &SqlitePool,
        mut files: Multipart,
        room: &str,
    ) -> Result<Vec<Self>, TapseError> {
        let mut new_files: Vec<Self> = Vec::new();

        while let Some(field) = files
            .next_field()
            .await
            .expect("Failed to get next multipart field")
        {
            // Randomly generated ID for the file.
            let id = nanoid::nanoid!(7);

            let filename = match field.file_name() {
                Some(a) => a.to_owned(),
                None => return Err(TapseError::FileName),
            };

            let bytes = field.bytes().await.unwrap().to_vec();

            // Disallow empty files
            if bytes.is_empty() {
                return Err(TapseError::FileEmpty);
            }

            new_files.push(
                sqlx::query_as!(
                    File,
                    r#"
                     insert into files (id, name, room, file, upload_date)
                     values ($1, $2, $3, $4, datetime('now'))
                     returning id, name, upload_date, room"#,
                    id,
                    filename,
                    room,
                    bytes
                )
                .fetch_one(pool)
                .await?,
            );
        }

        Ok(new_files)
    }

    pub async fn get(pool: &SqlitePool, file: FileQuery) -> Result<(Self, Vec<u8>), TapseError> {
        let q = sqlx::query!(
            r#"select * from files
               where id = $1 and name = $2"#,
            file.id,
            file.name
        )
        .fetch_one(pool)
        .await?;

        let file = Self {
            id: q.id,
            name: q.name,
            room: q.room,
            upload_date: q.upload_date,
        };

        Ok((file, q.file))
    }

    pub async fn move_files(
        pool: &SqlitePool,
        files: &FileMoveRequest,
    ) -> Result<Vec<File>, TapseError> {
        let mut moved_files = vec![];

        // yes, this could probably be done in one query
        for file in &files.move_files {
            let f = sqlx::query_as!(
                File,
                r#"
            update files
            set room = $1
            where id = $2
            and name = $3
            returning id, name, upload_date, room
        "#,
                files.new_room,
                file.id,
                file.name
            )
            .fetch_one(pool)
            .await?;

            moved_files.push(f);
        }

        Ok(moved_files)
    }

    pub(crate) async fn delete(
        pool: &SqlitePool,
        files: &Vec<FileQuery>,
    ) -> Result<Vec<FileQuery>, TapseError> {
        for file in files {
            sqlx::query!(
                r#"delete from files
                where id = $1 and name = $2"#,
                file.id,
                file.name
            )
            .execute(pool)
            .await?;
        }

        Ok(files.clone())
    }
}
