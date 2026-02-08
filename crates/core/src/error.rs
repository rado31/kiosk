use thiserror::Error;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] ureq::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Version parse error: {0}")]
    Version(#[from] semver::Error),

    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Base64 decode error")]
    Base64Decode,

    #[error("{0}")]
    Custom(String),
}

impl AppError {
    pub fn custom(msg: impl Into<String>) -> Self {
        Self::Custom(msg.into())
    }
}
