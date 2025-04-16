use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Debug, Serialize, Deserialize, thiserror::Error, Type)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum Error {
    #[error("An unexpected error occurred: {reason}")]
    Unknown{ reason: String },

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

    #[error("The requested context ({reason}) has not been initialized.")]
    UnknownContext{ reason: String },

    #[error("The requested database ({reason}) has not been opened.")]
    UnknownDatabase{ reason: String },

    #[error("The file handle with ID {reason} does not exist.")]
    UnknownFileHandle{ reason: String },

    #[error("Unknown transaction ID {reason} in current database.")]
    UnknownTransaction{ reason: String },

    #[error("Invalid path: {reason}")]
    InvalidPath{ reason: String },

    #[error("Cannot use an absolute path in this context: {reason}")]
    NoAbsolutePaths{ reason: String },

    #[error("Specified relative path escapes root path of this context: {reason}")]
    PathEscapesContext{ reason: String },

    #[error("Encountered a database error: {reason}")]
    DatabaseError{ reason: String },

    #[error("Serialization error: {reason}")]
    SerializationError{ reason: String },

    #[error("Deserialization error: {reason}")]
    DeserializationError{ reason: String },

    #[error("Encountered an IO error: {reason}")]
    IOError{ reason: String },

    #[error("Failed to encode {reason} bytes as UTF-8 string.")]
    StringEncodingError{ reason: String },

    #[error("Filesystem operation failed ({operation}): {reason}")]
    FilesystemError {operation: String, reason: String}
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Self::Unknown{reason: value.to_string()}
    }
}

impl From<polodb_core::Error> for Error {
    fn from(value: polodb_core::Error) -> Self {
        Self::DatabaseError{reason: value.to_string()}
    }
}

impl From<bson::ser::Error> for Error {
    fn from(value: bson::ser::Error) -> Self {
        Self::SerializationError{reason: value.to_string()}
    }
}

impl From<bson::de::Error> for Error {
    fn from(value: bson::de::Error) -> Self {
        Self::DeserializationError{reason: value.to_string()}
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IOError {reason: value.to_string()}
    }
}

impl Error {
    pub fn open_context(name: impl AsRef<str>, path: impl AsRef<str>, reason: impl AsRef<str>) -> Self {
        Self::OpenContext { name: name.as_ref().to_string(), path: path.as_ref().to_string(), reason: reason.as_ref().to_string() }
    }

    pub fn invalid_path(path: impl AsRef<str>) -> Self {
        Self::InvalidPath{reason: path.as_ref().to_string()}
    }

    pub fn unknown_context(name: impl AsRef<str>) -> Self {
        Self::UnknownContext{reason: name.as_ref().to_string()}
    }

    pub fn open_database(name: impl AsRef<str>, context: impl AsRef<str>, path: impl AsRef<str>, reason: impl AsRef<str>) -> Self {
        Self::OpenDatabase { name: name.as_ref().to_string(), context: context.as_ref().to_string(), path: path.as_ref().to_string(), reason: reason.as_ref().to_string() }
    }

    pub fn unknown_database(name: impl AsRef<str>) -> Self {
        Self::UnknownDatabase{reason: name.as_ref().to_string()}
    }

    pub fn no_absolute_path(path: impl AsRef<str>) -> Self {
        Self::NoAbsolutePaths{reason: path.as_ref().to_string()}
    }

    pub fn path_escapes_context(path: impl AsRef<str>) -> Self {
        Self::PathEscapesContext{reason: path.as_ref().to_string()}
    }

    pub fn open_file_handle(path: impl AsRef<str>, context: impl AsRef<str>, reason: impl AsRef<str>) -> Self {
        Self::OpenFileHandle { context: context.as_ref().to_string(), path: path.as_ref().to_string(), reason: reason.as_ref().to_string() }
    }

    pub fn unknown_file_handle(id: impl AsRef<str>) -> Self {
        Self::UnknownFileHandle{reason: id.as_ref().to_string()}
    }

    pub fn unknown_transaction(id: impl AsRef<str>) -> Self {
        Self::UnknownTransaction{reason: id.as_ref().to_string()}
    }

    pub fn string_encoding(size: usize) -> Self {
        Self::StringEncodingError{reason: size.to_string()}
    }

    pub fn filesystem(operation: impl AsRef<str>, reason: impl AsRef<str>) -> Self {
        Self::FilesystemError { operation: operation.as_ref().to_string(), reason: reason.as_ref().to_string() }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
