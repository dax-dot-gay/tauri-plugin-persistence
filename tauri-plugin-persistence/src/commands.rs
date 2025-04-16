use std::{collections::HashMap, ffi::OsString};

use polodb_core::{options::UpdateOptions, IndexModel, IndexOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{
    api::types::{
        CollectionSpecifier, ContextInfo, ContextSpecifier, DatabaseInfo, DatabaseSpecifier,
        FileHandleInfo, FileHandleSpecifier, OperationCount, UpdateResult,
    },
    PersistenceExt,
};

// Info commands
#[tauri::command]
#[specta::specta]
pub async fn context(
    app: tauri::AppHandle,
    context: ContextSpecifier,
) -> crate::Result<ContextInfo> {
    let context = app.persistence().context(context).await?;
    Ok(ContextInfo {
        name: context.name(),
        path: context.path(),
    })
}

#[tauri::command]
#[specta::specta]
pub async fn database(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    database: DatabaseSpecifier,
) -> crate::Result<DatabaseInfo> {
    let database = app.persistence().database(context, database).await?;
    Ok(DatabaseInfo {
        name: database.name(),
        path: database.path(),
    })
}

#[tauri::command]
#[specta::specta]
pub async fn file_handle(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    file_handle: FileHandleSpecifier,
) -> crate::Result<FileHandleInfo> {
    let file_handle = app.persistence().file_handle(context, file_handle).await?;
    Ok(FileHandleInfo {
        id: file_handle.id(),
        path: file_handle.path(),
        mode: file_handle.mode().await,
    })
}

// Database commands
#[tauri::command]
#[specta::specta]
pub async fn database_get_collections(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    database: DatabaseSpecifier,
) -> crate::Result<Vec<String>> {
    let database = app.persistence().database(context, database).await?;
    database.collections().await
}

#[tauri::command]
#[specta::specta]
pub async fn database_close(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    database: DatabaseSpecifier,
) -> crate::Result<()> {
    let database = app.persistence().database(context, database).await?;
    database.close().await
}

#[tauri::command]
#[specta::specta]
pub async fn database_start_transaction(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    database: DatabaseSpecifier,
) -> crate::Result<bson::Uuid> {
    let database = app.persistence().database(context, database).await?;
    Ok(database.start_transaction().await?.id())
}

#[tauri::command]
#[specta::specta]
pub async fn database_commit_transaction(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    database: DatabaseSpecifier,
    transaction: bson::Uuid,
) -> crate::Result<()> {
    let database = app.persistence().database(context, database).await?;
    database.commit_transaction(transaction).await
}

#[tauri::command]
#[specta::specta]
pub async fn database_rollback_transaction(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    database: DatabaseSpecifier,
    transaction: bson::Uuid,
) -> crate::Result<()> {
    let database = app.persistence().database(context, database).await?;
    database.rollback_transaction(transaction).await
}

// Collection commands
#[tauri::command]
#[specta::specta]
pub async fn collection_count_documents(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    database: DatabaseSpecifier,
    collection: CollectionSpecifier,
) -> crate::Result<u64> {
    let collection = app
        .persistence()
        .collection::<bson::Document>(context, database, collection)
        .await?;
    collection.count_documents().await
}

#[tauri::command]
#[specta::specta]
pub async fn collection_update_documents(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    database: DatabaseSpecifier,
    collection: CollectionSpecifier,
    query: crate::types::JsonDocument,
    update: crate::types::JsonDocument,
    operations: OperationCount,
    upsert: bool,
) -> crate::Result<UpdateResult> {
    let collection = app
        .persistence()
        .collection::<bson::Document>(context, database, collection)
        .await?;
    match operations {
        OperationCount::One => {
            collection
                .update_one_with_options(
                    query.into(),
                    update.into(),
                    UpdateOptions::builder().upsert(upsert).build(),
                )
                .await
        }
        OperationCount::Many => {
            collection
                .update_many_with_options(
                    query.into(),
                    update.into(),
                    UpdateOptions::builder().upsert(upsert).build(),
                )
                .await
        }
    }
    .and_then(|r| Ok(UpdateResult::from(r)))
}

#[tauri::command]
#[specta::specta]
pub async fn collection_delete_documents(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    database: DatabaseSpecifier,
    collection: CollectionSpecifier,
    query: crate::types::JsonDocument,
    operations: OperationCount
) -> crate::Result<u64> {
    let collection = app
        .persistence()
        .collection::<bson::Document>(context, database, collection)
        .await?;
    match operations {
        OperationCount::One => {
            collection
                .delete_one(
                    query.into()
                )
                .await
        }
        OperationCount::Many => {
            collection
                .delete_many(
                    query.into(),
                )
                .await
        }
    }
    .and_then(|r| Ok(r.deleted_count))
}

#[tauri::command]
#[specta::specta]
pub async fn collection_create_index(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    database: DatabaseSpecifier,
    collection: CollectionSpecifier,
    keys: crate::types::JsonDocument,
    name: Option<String>,
    unique: Option<bool>
) -> crate::Result<()> {
    let collection = app
        .persistence()
        .collection::<bson::Document>(context, database, collection)
        .await?;
    collection.create_index(IndexModel {keys: keys.into(), options: Some(IndexOptions {name, unique})}).await
}

#[tauri::command]
#[specta::specta]
pub async fn collection_drop_index(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    database: DatabaseSpecifier,
    collection: CollectionSpecifier,
    name: String
) -> crate::Result<()> {
    let collection = app
        .persistence()
        .collection::<bson::Document>(context, database, collection)
        .await?;
    collection.drop_index(name).await
}

#[tauri::command]
#[specta::specta]
pub async fn collection_drop(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    database: DatabaseSpecifier,
    collection: CollectionSpecifier
) -> crate::Result<()> {
    let collection = app
        .persistence()
        .collection::<bson::Document>(context, database, collection)
        .await?;
    collection.drop().await
}

#[tauri::command]
#[specta::specta]
pub async fn collection_insert_documents(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    database: DatabaseSpecifier,
    collection: CollectionSpecifier,
    documents: Vec<crate::types::JsonDocument>
) -> crate::Result<HashMap<usize, crate::types::JsonBson>> {
    let collection = app
        .persistence()
        .collection::<bson::Document>(context, database, collection)
        .await?;
    Ok(collection.insert_many(documents.iter().map(|v| <crate::types::JsonDocument as Into<bson::Document>>::into(v.clone())).collect::<Vec<bson::Document>>()).await?.inserted_ids.iter().map(|(k, v)| (k.clone(), crate::types::JsonBson::from(v.clone()))).collect())
}

#[tauri::command]
#[specta::specta]
pub async fn collection_find_many_documents(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    database: DatabaseSpecifier,
    collection: CollectionSpecifier,
    filter: crate::types::JsonDocument,
    skip: Option<u64>,
    limit: Option<u64>,
    sort: Option<crate::types::JsonDocument>
) -> crate::Result<Vec<crate::types::JsonDocument>> {
    let collection = app
        .persistence()
        .collection::<bson::Document>(context, database, collection)
        .await?;
    collection.find(filter.into(), skip, limit, sort.and_then(|i| Some(i.into()))).await.and_then(|r| Ok(r.iter().map(|i| crate::types::JsonDocument::from(i.clone())).collect()))
}

#[tauri::command]
#[specta::specta]
pub async fn collection_find_one_document(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    database: DatabaseSpecifier,
    collection: CollectionSpecifier,
    filter: crate::types::JsonDocument,
) -> crate::Result<Option<crate::types::JsonDocument>> {
    let collection = app
        .persistence()
        .collection::<bson::Document>(context, database, collection)
        .await?;
    collection.find_one(filter.into()).await.and_then(|r| Ok(r.and_then(|s| Some(s.into()))))
}

// File handle commands
#[tauri::command]
#[specta::specta]
pub async fn file_close(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    file_handle: FileHandleSpecifier
) -> crate::Result<()> {
    let file = app.persistence().file_handle(context, file_handle).await?;
    file.close().await
}

#[tauri::command]
#[specta::specta]
pub async fn file_write_text(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    file_handle: FileHandleSpecifier,
    data: String
) -> crate::Result<()> {
    let file = app.persistence().file_handle(context, file_handle).await?;
    let mutex_handle = file.handle().await;
    let mut handle = mutex_handle.lock();
    handle.write_all(data.as_bytes()).await.or_else(|e| Err(crate::Error::from(e)))
}

#[tauri::command]
#[specta::specta]
pub async fn file_write_bytes(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    file_handle: FileHandleSpecifier,
    data: Vec<u8>
) -> crate::Result<()> {
    let file = app.persistence().file_handle(context, file_handle).await?;
    let mutex_handle = file.handle().await;
    let mut handle = mutex_handle.lock();
    handle.write_all(&data).await.or_else(|e| Err(crate::Error::from(e)))
}

#[tauri::command]
#[specta::specta]
pub async fn file_read_text(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    file_handle: FileHandleSpecifier,
    size: Option<usize>
) -> crate::Result<String> {
    let file = app.persistence().file_handle(context, file_handle).await?;
    let mutex_handle = file.handle().await;
    let mut handle = mutex_handle.lock();
    let output: Vec<u8> = if let Some(sz) = size {
        let mut buffer: Box<[u8]> = vec![0; sz].into_boxed_slice();
        handle.read(&mut buffer).await.or_else(|e| Err(crate::Error::from(e)))?;
        buffer.to_vec()
    } else {
        let mut buffer: Vec<u8> = Vec::new();
        handle.read_to_end(&mut buffer).await.or_else(|e| Err(crate::Error::from(e)))?;
        buffer
    };
    let output_size = output.len();
    String::from_utf8(output).or_else(|_| Err(crate::Error::string_encoding(output_size)))
}

#[tauri::command]
#[specta::specta]
pub async fn file_read_bytes(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    file_handle: FileHandleSpecifier,
    size: Option<usize>
) -> crate::Result<Vec<u8>> {
    let file = app.persistence().file_handle(context, file_handle).await?;
    let mutex_handle = file.handle().await;
    let mut handle = mutex_handle.lock();
    let output: Vec<u8> = if let Some(sz) = size {
        let mut buffer: Box<[u8]> = vec![0; sz].into_boxed_slice();
        handle.read(&mut buffer).await.or_else(|e| Err(crate::Error::from(e)))?;
        buffer.to_vec()
    } else {
        let mut buffer: Vec<u8> = Vec::new();
        handle.read_to_end(&mut buffer).await.or_else(|e| Err(crate::Error::from(e)))?;
        buffer
    };
    Ok(output)
}

// Filesystem commands
#[tauri::command]
#[specta::specta]
pub async fn get_context_base_path(
    app: tauri::AppHandle,
    context: ContextSpecifier
) -> crate::Result<OsString> {
    let context = app.persistence().context(context).await?;
    context.base_path_canonicalized().and_then(|p| Ok(p.into_os_string()))
}

#[tauri::command]
#[specta::specta]
pub async fn create_directory(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    path: String,
    parents: bool
) -> crate::Result<()> {
    let context = app.persistence().context(context).await?;
    context.create_directory(path, parents).await
}

#[tauri::command]
#[specta::specta]
pub async fn remove_directory(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    path: String
) -> crate::Result<()> {
    let context = app.persistence().context(context).await?;
    context.remove_directory(path).await
}

#[tauri::command]
#[specta::specta]
pub async fn remove_file(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    path: String
) -> crate::Result<()> {
    let context = app.persistence().context(context).await?;
    context.remove_file(path).await
}

#[tauri::command]
#[specta::specta]
pub async fn get_absolute_path_to(
    app: tauri::AppHandle,
    context: ContextSpecifier,
    path: String
) -> crate::Result<OsString> {
    let context = app.persistence().context(context).await?;
    Ok(context.get_path(path)?.into_os_string())
}