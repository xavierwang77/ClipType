use thiserror::Error;

pub type Result<T> = std::result::Result<T, ClipTypeError>;

#[derive(Debug, Error)]
pub enum ClipTypeError {
    #[error("clipboard error: {0}")]
    Clipboard(String),
    #[error("hotkey error: {0}")]
    Hotkey(String),
    #[error("input error: {0}")]
    Input(String),
    #[error("permission error: {0}")]
    Permission(String),
    #[error("application state error: {0}")]
    State(String),
    #[error("platform error: {0}")]
    Platform(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Tauri(#[from] tauri::Error),
    #[error(transparent)]
    TomlDeserialize(#[from] toml::de::Error),
    #[error(transparent)]
    TomlSerialize(#[from] toml::ser::Error),
}
