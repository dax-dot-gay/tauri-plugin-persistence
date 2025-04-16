use serde::{Deserialize, Serialize};
use specta::Type;

use super::state::FileHandleMode;

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(untagged)]
pub enum ContextSpecifier {
    Aliased { alias: String },
    Direct { alias: String, path: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct ContextInfo {
    pub name: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(untagged)]
pub enum DatabaseSpecifier {
    Aliased { alias: String },
    Direct { alias: String, path: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct DatabaseInfo {
    pub name: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(untagged)]
pub enum FileHandleSpecifier {
    Aliased { id: bson::Uuid },
    Direct { path: String, mode: FileHandleMode },
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct FileHandleInfo {
    pub id: bson::Uuid,
    pub path: String,
    pub mode: FileHandleMode,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(untagged)]
pub enum CollectionSpecifier {
    Global {
        name: String,
    },
    Transaction {
        transaction: bson::Uuid,
        name: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct CollectionInfo {
    pub database: DatabaseInfo,
    pub name: String,
    pub transaction_id: Option<bson::Uuid>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(rename_all = "snake_case")]
pub enum OperationCount {
    One,
    Many,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct UpdateResult {
    #[specta(type = u32)]
    pub matched: u64,
    #[specta(type = u32)]
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
