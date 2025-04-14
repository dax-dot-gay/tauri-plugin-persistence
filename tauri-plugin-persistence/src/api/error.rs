use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, thiserror::Error)]
pub enum Error {
    #[error("An unexpected error occurred: {0}")]
    Unknown(String),
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Self::Unknown(value.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
