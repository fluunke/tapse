use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use thiserror::Error;
use tracing::error;

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
    #[error("Invalid WebSocket message")]
    InvalidQuery,
    #[error("Wrong password")]
    WrongPassword,
    #[error("Database error. If this problem persists, contact the administrator.\nError: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Session not found.")]
    SessionNotFound,
}

impl IntoResponse for TapseError {
    fn into_response(self) -> Response {
        let res = match self {
            Self::WrongPassword | Self::SessionNotFound => (StatusCode::UNAUTHORIZED, self),

            Self::NoMessages | Self::NoFiles => (StatusCode::NO_CONTENT, self),

            Self::FileName
            | Self::FileEmpty
            | Self::InvalidQuery
            | Self::MessageLength
            | Self::RoomNameLength => (StatusCode::BAD_REQUEST, self),

            Self::DatabaseError(e) => {
                error!("{}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    TapseError::DatabaseError(e),
                )
            }
        };

        res.into_response()
    }
}
