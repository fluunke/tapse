use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TapseError {
    #[error("No messages were found for this room")]
    NoMessages,
    #[error("No files were uploaded yet")]
    NoFiles,
    #[error("Invalid filename")]
    FileName,
    #[error("File is empty")]
    FileEmpty,
    #[error("Message must be between 1 and 1024 characters long")]
    MessageLength,
    #[error("Room name must be between 3 and 20 characters long")]
    RoomNameLength,
    #[error("Error while creating the room: {0}")]
    RoomCreationError(sqlx::Error),
    #[error("Invalid WebSocket message")]
    InvalidQuery,
    #[error("Wrong password")]
    WrongPassword,
    #[error("Database error. If this problem persists, contact the administrator.\nError: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

impl IntoResponse for TapseError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
