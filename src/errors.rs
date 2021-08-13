use thiserror::Error;
#[derive(Error, Debug)]
pub enum TapseError {
    #[error("No messages were found for this room.")]
    NoMessages,
    #[error("No files were uploaded yet.")]
    NoFiles,
    #[error("Invalid filename.")]
    FileName,
    #[error("File is empty.")]
    FileEmpty,
    #[error("Message too short.")]
    MessageTooShort,
    #[error("Message too long.")]
    MessageTooLong,
    #[error("Room name is too short.")]
    RoomNameTooShort,
    #[error("Room name is too long")]
    RoomNameTooLong,
    #[error("There was an error while creating the room: {0}")]
    RoomCreationError(sqlx::Error),
    #[error("Database error. If this problem persists, contact the administrator.\nError: {0}")]
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for TapseError {
    fn from(e: sqlx::Error) -> Self {
        TapseError::DatabaseError(e)
    }
}
