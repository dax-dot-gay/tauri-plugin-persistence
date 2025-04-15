use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, thiserror::Error)]
pub enum Error {
    #[error("An unexpected error occurred: {0}")]
    Unknown(String),

    #[error("Failed to open context {name} at {path}: {reason}")]
    OpenContext {
        name: String,
        path: String,
        reason: String
    },

    #[error("Failed to open database {name} in context {context} at {path}: {reason}")]
    OpenDatabase {
        name: String,
        context: String,
        path: String,
        reason: String
    },

    #[error("Failed to open {path} in {context}: {reason}")]
    OpenFileHandle {
        path: String,
        context: String,
        reason: String
    },

    #[error("The requested context ({0}) has not been initialized.")]
    UnknownContext(String),

    #[error("The requested database ({0}) has not been opened.")]
    UnknownDatabase(String),

    #[error("The file handle with ID {0} does not exist.")]
    UnknownFileHandle(String),

    #[error("Unknown transaction ID {0} in current database.")]
    UnknownTransaction(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Cannot use an absolute path in this context: {0}")]
    NoAbsolutePaths(String),

    #[error("Specified relative path escapes root path of this context: {0}")]
    PathEscapesContext(String),

    #[error("Encountered a database error: {0}")]
    DatabaseError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String)
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Self::Unknown(value.to_string())
    }
}

impl From<polodb_core::Error> for Error {
    fn from(value: polodb_core::Error) -> Self {
        Self::DatabaseError(value.to_string())
    }
}

impl From<bson::ser::Error> for Error {
    fn from(value: bson::ser::Error) -> Self {
        Self::SerializationError(value.to_string())
    }
}

impl From<bson::de::Error> for Error {
    fn from(value: bson::de::Error) -> Self {
        Self::DeserializationError(value.to_string())
    }
}

impl Error {
    pub fn open_context(name: impl AsRef<str>, path: impl AsRef<str>, reason: impl AsRef<str>) -> Self {
        Self::OpenContext { name: name.as_ref().to_string(), path: path.as_ref().to_string(), reason: reason.as_ref().to_string() }
    }

    pub fn invalid_path(path: impl AsRef<str>) -> Self {
        Self::InvalidPath(path.as_ref().to_string())
    }

    pub fn unknown_context(name: impl AsRef<str>) -> Self {
        Self::UnknownContext(name.as_ref().to_string())
    }

    pub fn open_database(name: impl AsRef<str>, context: impl AsRef<str>, path: impl AsRef<str>, reason: impl AsRef<str>) -> Self {
        Self::OpenDatabase { name: name.as_ref().to_string(), context: context.as_ref().to_string(), path: path.as_ref().to_string(), reason: reason.as_ref().to_string() }
    }

    pub fn unknown_database(name: impl AsRef<str>) -> Self {
        Self::UnknownDatabase(name.as_ref().to_string())
    }

    pub fn no_absolute_path(path: impl AsRef<str>) -> Self {
        Self::NoAbsolutePaths(path.as_ref().to_string())
    }

    pub fn path_escapes_context(path: impl AsRef<str>) -> Self {
        Self::PathEscapesContext(path.as_ref().to_string())
    }

    pub fn open_file_handle(path: impl AsRef<str>, context: impl AsRef<str>, reason: impl AsRef<str>) -> Self {
        Self::OpenFileHandle { context: context.as_ref().to_string(), path: path.as_ref().to_string(), reason: reason.as_ref().to_string() }
    }

    pub fn unknown_file_handle(id: impl AsRef<str>) -> Self {
        Self::UnknownFileHandle(id.as_ref().to_string())
    }

    pub fn unknown_transaction(id: impl AsRef<str>) -> Self {
        Self::UnknownTransaction(id.as_ref().to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
