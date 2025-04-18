use std::fs::{FileType, Metadata};

use chrono::Utc;
use mime_guess::MimeGuess;
use serde::{Deserialize, Serialize};
use specta::Type;
use tokio::fs::DirEntry;

use super::state::FileHandleMode;

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(untagged)]
/// A model used to specify an existing or closed context
pub enum ContextSpecifier {
    /// Open a new context
    Direct { alias: String, path: String },

    /// Return an existing context
    Aliased { alias: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
/// A model containing serializable information about a [crate::Context]
pub struct ContextInfo {
    /// Context name
    pub name: String,

    /// Context path
    pub path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(untagged)]
/// A model used to specify an existing or closed database
pub enum DatabaseSpecifier {
    /// Open a new database
    Direct { alias: String, path: String },

    /// Return an existing database
    Aliased { alias: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
/// A model containing serializable information about a [crate::Database]
pub struct DatabaseInfo {
    /// Database name
    pub name: String,

    /// Database path
    pub path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(untagged)]
/// A model used to specify an existing or closed file handle
pub enum FileHandleSpecifier {
    /// Return an existing file handle
    Aliased { id: bson::Uuid },

    /// Open a new file handle
    Direct { path: String, mode: FileHandleMode },
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
/// A model containing serializable information about a [crate::FileHandle]
pub struct FileHandleInfo {
    /// File handle ID
    pub id: bson::Uuid,

    /// File handle path
    pub path: String,

    /// Open mode
    pub mode: FileHandleMode,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(untagged)]
/// A model used to specify a collection
pub enum CollectionSpecifier {
    /// Open a collection in a transaction
    Transaction {
        /// Transaction ID
        transaction: bson::Uuid,

        /// Collection name
        name: String,
    },

    /// Open a non-transacted collection
    Global {

        /// Collection name
        name: String,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
/// A model containing serializable information about a [crate::Collection]
pub struct CollectionInfo {
    /// Database info
    pub database: DatabaseInfo,

    /// Collection name
    pub name: String,

    /// Transaction ID
    pub transaction_id: Option<bson::Uuid>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(rename_all = "snake_case")]
/// Whether to do one operation or multiple (in a database context)
pub enum OperationCount {
    ///
    One,
    ///
    Many,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
/// Serializable version of [polodb_core::results::UpdateResult]
pub struct UpdateResult {
    #[specta(type = u32)]
    /// How many documents matched the filter
    pub matched: u64,
    #[specta(type = u32)]
    /// How many documents were updated
    pub modified: u64,
}

impl From<polodb_core::results::UpdateResult> for UpdateResult {
    fn from(value: polodb_core::results::UpdateResult) -> Self {
        UpdateResult {
            matched: value.matched_count,
            modified: value.modified_count,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
/// Specta version of [bson::Bson]
pub struct JsonBson(bson::Bson);

impl Type for JsonBson {
    fn inline(
        type_map: &mut specta::TypeCollection,
        generics: specta::Generics,
    ) -> specta::datatype::DataType {
        serde_json::Value::inline(type_map, generics)
    }
}

impl From<bson::Bson> for JsonBson {
    fn from(value: bson::Bson) -> Self {
        Self(value)
    }
}

impl Into<bson::Bson> for JsonBson {
    fn into(self) -> bson::Bson {
        self.0.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
/// Specta version of [bson::Document]
pub struct JsonDocument(bson::Document);

impl Type for JsonDocument {
    fn inline(
        type_map: &mut specta::TypeCollection,
        generics: specta::Generics,
    ) -> specta::datatype::DataType {
        serde_json::Value::inline(type_map, generics)
    }
}

impl From<bson::Document> for JsonDocument {
    fn from(value: bson::Document) -> Self {
        Self(value)
    }
}

impl Into<bson::Document> for JsonDocument {
    fn into(self) -> bson::Document {
        self.0.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(rename_all = "snake_case")]
/// Description of the type of a file/directory/symlink
pub enum PathFileType {
    ///
    Directory,
    ///
    File,
    ///
    Symlink
}

impl From<FileType> for PathFileType {
    fn from(value: FileType) -> Self {
        if value.is_dir() {
            Self::Directory
        } else if value.is_file() {
            Self::File
        } else {
            Self::Symlink
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
/// File or folder metadata
pub struct PathMetadata {
    pub file_type: PathFileType,
    pub size: u64,
    pub last_modified: Option<chrono::DateTime<Utc>>,
    pub last_accessed: Option<chrono::DateTime<Utc>>,
    pub created: Option<chrono::DateTime<Utc>>,
}

impl From<Metadata> for PathMetadata {
    fn from(value: Metadata) -> Self {
        Self {
            file_type: PathFileType::from(value.file_type()),
            size: value.len(),
            last_modified: value.modified().and_then(|systime| Ok(Some(chrono::DateTime::<Utc>::from(systime)))).unwrap_or(None),
            last_accessed: value.accessed().and_then(|systime| Ok(Some(chrono::DateTime::<Utc>::from(systime)))).unwrap_or(None),
            created: value.created().and_then(|systime| Ok(Some(chrono::DateTime::<Utc>::from(systime)))).unwrap_or(None)
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
/// General info about a path
pub struct PathInformation {
    pub file_name: String,
    pub absolute_path: String,
    pub media_type: String,
}

impl From<DirEntry> for PathInformation {
    fn from(value: DirEntry) -> Self {
        Self {
            file_name: String::from_utf8_lossy(value.file_name().as_encoded_bytes()).to_string(),
            absolute_path: String::from_utf8_lossy(value.path().into_os_string().as_encoded_bytes()).to_string(),
            media_type: MimeGuess::from_path(value.path()).first_or_octet_stream().to_string()
        }
    }
}