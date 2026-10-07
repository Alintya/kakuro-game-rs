use serde::Serialize;
use specta::Type;

/// Application-wide error type, also used at the IPC boundary.
///
/// Serialized to the frontend as `{ kind, message? }`.
#[derive(Debug, thiserror::Error, Serialize, Type)]
#[serde(tag = "kind", content = "message")]
pub enum AppError {
    #[error("no game in progress")]
    NoGame,

    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("io: {0}")]
    Io(String),

    #[error("{0}")]
    Internal(String),
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.to_string())
    }
}
