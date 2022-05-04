use std::convert::TryFrom;

use serde::{Deserialize, Serialize};
use serde_json::from_str;
use sqlx::{Pool, Sqlite};

use crate::{
    db::{File, FileQuery, Message, MessageQuery, Room, RoomQuery},
    errors::TapseError,
    handlers::session::User,
};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
/// Request from the client
/// {"new_message":{"room":"general","content":"hello!"}}
pub enum Request {
    NewMessage(MessageQuery),
    CreateRoom(RoomQuery),
    MoveFiles(FileMove),
    DeleteFiles(Vec<FileQuery>),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Response {
    Error(WSError),
    IncomingMessage(Message),
    NewFiles(Vec<File>),
    RoomCreated(Room),
    FilesMoved(FileMove),
    FilesDeleted(Vec<FileQuery>),
}

impl Response {
    pub fn error(recipient_id: &str, message: &TapseError) -> Self {
        Self::Error(WSError {
            recipient_id: recipient_id.to_string(),
            message: message.to_string(),
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WSError {
    #[serde(skip_serializing)]
    pub recipient_id: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileMove {
    pub move_files: Vec<FileQuery>,
    pub new_room: String,
}

impl TryFrom<&str> for Request {
    type Error = TapseError;

    fn try_from(s: &str) -> Result<Self, TapseError> {
        from_str::<Self>(s).map_err(|_| TapseError::InvalidQuery)
    }
}

impl Request {
    pub async fn execute(input: &str, user: &User, pool: &Pool<Sqlite>) -> Response {
        let event = match Self::try_from(input) {
            Ok(event) => event,
            Err(e) => return Response::error(&user.id, &e),
        };

        let returned_event: Response = match event {
            Self::NewMessage(msg) => match Message::add(pool, &user.username, &msg).await {
                Ok(new) => Response::IncomingMessage(new),
                Err(e) => Response::error(&user.id, &e),
            },
            Self::CreateRoom(room) => match Room::new(pool, &room).await {
                Ok(new) => Response::RoomCreated(new),
                Err(e) => Response::error(&user.id, &e),
            },
            Self::MoveFiles(files) => match File::move_files(pool, &files).await {
                Ok(_) => Response::FilesMoved(files),
                Err(e) => Response::error(&user.id, &e),
            },
            Self::DeleteFiles(files) => match File::delete(pool, &files).await {
                Ok(ids) => Response::FilesDeleted(ids),
                Err(e) => Response::error(&user.id, &e),
            },
        };

        returned_event
    }
}
