use std::{borrow::Borrow, collections::HashMap, marker::PhantomData, ops::Deref, path::PathBuf, str::FromStr, sync::Arc};

use bson::Document;
use polodb_core::{options::UpdateOptions, results::{DeleteResult, InsertManyResult, InsertOneResult, UpdateResult}, CollectionT, IndexModel};
use serde::{de::DeserializeOwned, Serialize};
use tauri::{AppHandle, Manager, Runtime};
use tokio::{fs::{File, OpenOptions}, sync::Mutex};

use super::state::{ContextDB, ContextFileHandle, ContextState, FileHandleMode, PluginState};

pub struct Context<R: Runtime> {
    handle: Arc<AppHandle<R>>,
    name: String,
    path: String,
}

impl<R: Runtime> Clone for Context<R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
            name: self.name.clone(),
            path: self.path.clone(),
        }
    }
}

impl<R: Runtime> Context<R> {
    pub(crate) fn create(handle: AppHandle<R>, name: String, path: String) -> Self {
        Self {
            handle: Arc::new(handle),
            name,
            path,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub(crate) fn handle(&self) -> AppHandle<R> {
        self.handle.clone().deref().clone()
    }

    pub fn base_path(&self) -> PathBuf {
        PathBuf::from_str(&self.path()).unwrap()
    }

    pub fn get_path(&self, path: impl AsRef<str>) -> crate::Result<PathBuf> {
        let resolved = PathBuf::from_str(path.as_ref()).unwrap();
        if resolved.is_absolute() {
            return Err(crate::Error::no_absolute_path(path.as_ref()));
        }
        let joined = self
            .base_path()
            .canonicalize().or(Err(crate::Error::invalid_path(path.as_ref())))?
            .join(&resolved);
        if !joined.starts_with(resolved) {
            return Err(crate::Error::path_escapes_context(path.as_ref()));
        }

        Ok(joined)
    }

    pub fn base_path_canonicalized(&self) -> crate::Result<PathBuf> {
        PathBuf::from_str(&self.path()).unwrap().canonicalize().or_else(|_| Err(crate::Error::invalid_path(self.path())))
    }

    pub async fn create_directory(&self, path: impl AsRef<str>, parents: bool) -> crate::Result<()> {
        let resolved = self.get_path(path)?;
        let create_result = if parents {tokio::fs::create_dir_all(&resolved).await} else {tokio::fs::create_dir(&resolved).await};
        if let Err(error) = create_result {
            return Err(crate::Error::filesystem("CREATE_DIRECTORY", error.to_string()));
        }

        Ok(())
    }

    pub async fn remove_directory(&self, path: impl AsRef<str>) -> crate::Result<()> {
        let resolved = self.get_path(path)?;
        if !resolved.is_dir() {
            return Err(crate::Error::filesystem("REMOVE_DIRECTORY", "Specified path is not a directory or does not exist."));
        }
        tokio::fs::remove_dir_all(resolved).await.or_else(|error| Err(crate::Error::filesystem("REMOVE_DIRECTORY", error.to_string())))?;
        Ok(())
    }

    pub async fn remove_file(&self, path: impl AsRef<str>) -> crate::Result<()> {
        let resolved = self.get_path(path)?;
        if !resolved.is_file() {
            return Err(crate::Error::filesystem("REMOVE_FILE", "Specified path is not a file or does not exist."));
        }
        tokio::fs::remove_file(resolved).await.or_else(|error| Err(crate::Error::filesystem("REMOVE_FILE", error.to_string())))?;
        Ok(())
    }

    pub(crate) async fn state(&self) -> ContextState {
        self.handle()
            .state::<PluginState>()
            .lock()
            .await
            .get(&self.name())
            .expect("Context not initialized.")
            .clone()
    }

    pub(crate) async fn databases(&self) -> Arc<Mutex<HashMap<String, ContextDB>>> {
        self.state().await.databases.clone()
    }

    pub(crate) async fn files(&self) -> Arc<Mutex<HashMap<bson::Uuid, ContextFileHandle>>> {
        self.state().await.files.clone()
    }

    pub async fn open_database(
        &self,
        name: impl AsRef<str>,
        path: impl AsRef<str>,
    ) -> crate::Result<Database<R>> {
        let _dbs = self.databases().await;
        let mut dbs = _dbs.lock().await;
        let resolved_path = self.get_path(path.as_ref())?;
        if let Some(db) = dbs.get(&name.as_ref().to_string()) {
            if db.path == path.as_ref().to_string() {
                Ok(Database::<R>::create(
                    self.clone(),
                    name.as_ref().to_string(),
                    path.as_ref().to_string(),
                ))
            } else {
                Err(crate::Error::open_database(
                    name.as_ref(),
                    self.name(),
                    path.as_ref(),
                    "Database is already open at another path.",
                ))
            }
        } else {
            if resolved_path.exists() {
                if resolved_path.is_file() {
                    let database =
                        polodb_core::Database::open_path(resolved_path).or_else(|e| {
                            Err(crate::Error::open_database(
                                name.as_ref(),
                                self.name(),
                                path.as_ref(),
                                e.to_string(),
                            ))
                        })?;
                    let _ = dbs.insert(
                        name.as_ref().to_string(),
                        ContextDB {
                            name: name.as_ref().to_string(),
                            path: path.as_ref().to_string(),
                            database: Arc::new(Mutex::new(database)),
                            transactions: Arc::new(Mutex::new(HashMap::new())),
                        },
                    );
                    Ok(Database::<R>::create(
                        self.clone(),
                        name.as_ref().to_string(),
                        path.as_ref().to_string(),
                    ))
                } else {
                    Err(crate::Error::open_database(
                        name.as_ref(),
                        self.name(),
                        path.as_ref(),
                        "Specified path is not a file.",
                    ))
                }
            } else {
                let database = polodb_core::Database::open_path(resolved_path).or_else(|e| {
                    Err(crate::Error::open_database(
                        name.as_ref(),
                        self.name(),
                        path.as_ref(),
                        e.to_string(),
                    ))
                })?;
                let _ = dbs.insert(
                    name.as_ref().to_string(),
                    ContextDB {
                        name: name.as_ref().to_string(),
                        path: path.as_ref().to_string(),
                        database: Arc::new(Mutex::new(database)),
                        transactions: Arc::new(Mutex::new(HashMap::new())),
                    },
                );
                Ok(Database::<R>::create(
                    self.clone(),
                    name.as_ref().to_string(),
                    path.as_ref().to_string(),
                ))
            }
        }
    }

    pub async fn database(&self, name: impl AsRef<str>) -> crate::Result<Database<R>> {
        if let Some(db) = self
            .databases()
            .await
            .lock()
            .await
            .get(&name.as_ref().to_string())
        {
            Ok(Database::<R>::create(
                self.clone(),
                name.as_ref().to_string(),
                db.path.clone(),
            ))
        } else {
            Err(crate::Error::unknown_database(name.as_ref()))
        }
    }

    pub(crate) async fn close_database(&self, name: impl AsRef<str>) -> crate::Result<()> {
        if let Some(_) = self
            .databases()
            .await
            .lock()
            .await
            .remove(&name.as_ref().to_string())
        {
            Ok(())
        } else {
            Err(crate::Error::unknown_database(name.as_ref()))
        }
    }

    pub async fn open_file_handle(
        &self,
        path: impl AsRef<str>,
        mode: FileHandleMode,
    ) -> crate::Result<FileHandle<R>> {
        let resolved = self.get_path(path.as_ref())?;
        if mode.create() && !resolved.exists() && resolved.clone().parent().is_some() {
            tokio::fs::create_dir_all(resolved.clone().parent().unwrap())
                .await
                .or_else(|e| {
                    Err(crate::Error::open_file_handle(
                        path.as_ref(),
                        self.name(),
                        e.to_string(),
                    ))
                })?;
        }

        let options: OpenOptions = mode.clone().into();
        let file = options.open(resolved.clone()).await.or_else(|e| {
            Err(crate::Error::open_file_handle(
                path.as_ref(),
                self.name(),
                e.to_string(),
            ))
        })?;
        let handle = ContextFileHandle {
            id: bson::Uuid::new(),
            path: path.as_ref().to_string(),
            handle: async_dup::Arc::new(async_dup::Mutex::new(file)),
            mode: mode.clone(),
        };
        let id = handle.id.clone();

        let _files = self.files().await;
        let mut files = _files.lock().await;
        let _ = files.insert(id.clone(), handle);
        Ok(FileHandle::<R>::create(
            self.clone(),
            id.clone(),
            path.as_ref().to_string(),
        ))
    }

    pub async fn file_handle(&self, id: bson::Uuid) -> crate::Result<FileHandle<R>> {
        if let Some(handle) = self.files().await.lock().await.get(&id) {
            Ok(FileHandle::<R>::create(
                self.clone(),
                id.clone(),
                handle.path.clone(),
            ))
        } else {
            Err(crate::Error::unknown_file_handle(id.to_string()))
        }
    }

    pub(crate) async fn close_file_handle(&self, id: bson::Uuid) -> crate::Result<()> {
        if let Some(_) = self.files().await.lock().await.remove(&id) {
            Ok(())
        } else {
            Err(crate::Error::unknown_file_handle(id.to_string()))
        }
    }
}

pub struct Database<R: Runtime> {
    context: Context<R>,
    name: String,
    path: String,
}

impl<R: Runtime> Clone for Database<R> {
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
            name: self.name.clone(),
            path: self.path.clone()
        }
    }
}

impl<R: Runtime> Database<R> {
    pub(crate) fn create(context: Context<R>, name: String, path: String) -> Self {
        Self {
            context,
            name,
            path,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn absolute_path(&self) -> crate::Result<PathBuf> {
        self.context.get_path(self.path())
    }

    pub(crate) async fn db_context(&self) -> crate::Result<ContextDB> {
        if let Some(db) = self.context.databases().await.lock().await.get(&self.name) {
            Ok(db.clone())
        } else {
            Err(crate::Error::unknown_database(self.name()))
        }
    }

    pub(crate) async fn db(&self) -> crate::Result<Arc<Mutex<polodb_core::Database>>> {
        Ok(self.db_context().await?.database.clone())
    }

    pub async fn close(self) -> crate::Result<()> {
        self.context.close_database(self.name()).await
    }

    pub async fn collections(&self) -> crate::Result<Vec<String>> {
        let db = self.db().await?;
        let database = db.lock().await;
        Ok(database.list_collection_names().or_else(|e| Err(crate::Error::from(e)))?)
    }

    pub async fn collection<T: Serialize + DeserializeOwned + Send + Sync>(&self, name: impl AsRef<str>) -> Collection<T, R> {
        Collection::<T, R>::create(self.clone(), name.as_ref().to_string(), None)
    }

    pub async fn start_transaction(&self) -> crate::Result<Transaction<R>> {
        let context = self.db_context().await?;
        let db = context.database.lock().await;
        let mut transactions = context.transactions.lock().await;
        let new_id = bson::Uuid::new();
        transactions.insert(new_id.clone(), Arc::new(Mutex::new(db.start_transaction().or_else(|e| Err(crate::Error::from(e)))?)));
        Ok(Transaction::<R>::create(self.clone(), new_id))
    }

    pub async fn get_transaction(&self, id: bson::Uuid) -> crate::Result<Transaction<R>> {
        let context = self.db_context().await?;
        let transactions = context.transactions.lock().await;
        if let Some(_) = transactions.get(&id) {
            Ok(Transaction::<R>::create(self.clone(), id.clone()))
        } else {
            Err(crate::Error::unknown_transaction(id.to_string()))
        }
    }

    pub async fn commit_transaction(&self, id: bson::Uuid) -> crate::Result<()> {
        if let Some(mutex) = self.db_context().await?.transactions.lock().await.remove(&id) {
            let transaction = mutex.lock().await;
            transaction.commit().or_else(|e| Err(crate::Error::from(e)))
        } else {
            Err(crate::Error::unknown_transaction(id.to_string()))
        }
    }

    pub async fn rollback_transaction(&self, id: bson::Uuid) -> crate::Result<()> {
        if let Some(mutex) = self.db_context().await?.transactions.lock().await.remove(&id) {
            let transaction = mutex.lock().await;
            transaction.rollback().or_else(|e| Err(crate::Error::from(e)))
        } else {
            Err(crate::Error::unknown_transaction(id.to_string()))
        }
    }
}

pub struct Transaction<R: Runtime> {
    database: Database<R>,
    id: bson::Uuid
}

impl<R: Runtime> Clone for Transaction<R> {
    fn clone(&self) -> Self {
        Self {
            database: self.database.clone(),
            id: self.id.clone()
        }
    }
}

impl<R: Runtime> Transaction<R> {
    pub(crate) fn create(database: Database<R>, id: bson::Uuid) -> Self {
        Self {
            database, id
        }
    }

    pub fn id(&self) -> bson::Uuid {
        self.id.clone()
    }

    pub fn collection<T: Serialize + DeserializeOwned + Send + Sync>(&self, name: impl AsRef<str>) -> Collection<T, R> {
        Collection::create(self.database.clone(), name.as_ref().to_string(), Some(self.id.clone()))
    }

    pub async fn commit(self) -> crate::Result<()> {
        self.database.commit_transaction(self.id()).await
    }

    pub async fn rollback(self) -> crate::Result<()> {
        self.database.rollback_transaction(self.id()).await
    }
}

pub struct FileHandle<R: Runtime> {
    context: Context<R>,
    id: bson::Uuid,
    path: String,
}

impl<R: Runtime> Clone for FileHandle<R> {
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
            id: self.id.clone(),
            path: self.path.clone()
        }
    }
}

impl<R: Runtime> FileHandle<R> {
    pub(crate) fn create(context: Context<R>, id: bson::Uuid, path: String) -> Self {
        Self { context, id, path }
    }

    pub fn id(&self) -> bson::Uuid {
        self.id.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn absolute_path(&self) -> crate::Result<PathBuf> {
        self.context.get_path(self.path())
    }

    pub async fn close(self) -> crate::Result<()> {
        self.context.close_file_handle(self.id()).await
    }

    async fn metadata(&self) -> ContextFileHandle {
        self.context.files().await.lock().await.get(&self.id()).expect("File handle has been closed.").clone()
    }

    pub async fn mode(&self) -> FileHandleMode {
        self.metadata().await.mode
    }

    pub async fn handle(&self) -> async_dup::Arc<async_dup::Mutex<File>> {
        self.metadata().await.handle.clone()
    }
}

pub(crate) enum CollectionType {
    Standalone(polodb_core::Collection<Document>),
    Transaction(polodb_core::TransactionalCollection<Document>)
}

impl CollectionT<Document> for CollectionType {
    fn name(&self) -> &str {
        match self {
            Self::Standalone(c) => c.name(),
            Self::Transaction(c) => c.name()
        }
    }

    fn count_documents(&self) -> polodb_core::Result<u64> {
        match self {
            Self::Standalone(c) => c.count_documents(),
            Self::Transaction(c) => c.count_documents()
        }
    }

    fn update_one(&self, query: Document, update: Document) -> polodb_core::Result<polodb_core::results::UpdateResult> {
        match self {
            Self::Standalone(c) => c.update_one(query, update),
            Self::Transaction(c) => c.update_one(query, update)
        }
    }

    fn update_one_with_options(&self, query: Document, update: Document, options: polodb_core::options::UpdateOptions) -> polodb_core::Result<polodb_core::results::UpdateResult> {
        match self {
            Self::Standalone(c) => c.update_one_with_options(query, update, options),
            Self::Transaction(c) => c.update_one_with_options(query, update, options)
        }
    }

    fn update_many(&self, query: Document, update: Document) -> polodb_core::Result<polodb_core::results::UpdateResult> {
        match self {
            Self::Standalone(c) => c.update_many(query, update),
            Self::Transaction(c) => c.update_many(query, update)
        }
    }

    fn update_many_with_options(&self, query: Document, update: Document, options: polodb_core::options::UpdateOptions) -> polodb_core::Result<polodb_core::results::UpdateResult> {
        match self {
            Self::Standalone(c) => c.update_many_with_options(query, update, options),
            Self::Transaction(c) => c.update_many_with_options(query, update, options)
        }
    }

    fn delete_one(&self, query: Document) -> polodb_core::Result<polodb_core::results::DeleteResult> {
        match self {
            Self::Standalone(c) => c.delete_one(query),
            Self::Transaction(c) => c.delete_one(query)
        }
    }

    fn delete_many(&self, query: Document) -> polodb_core::Result<polodb_core::results::DeleteResult> {
        match self {
            Self::Standalone(c) => c.delete_many(query),
            Self::Transaction(c) => c.delete_many(query)
        }
    }

    fn create_index(&self, index: polodb_core::IndexModel) -> polodb_core::Result<()> {
        match self {
            Self::Standalone(c) => c.create_index(index),
            Self::Transaction(c) => c.create_index(index)
        }
    }

    fn drop_index(&self, name: impl AsRef<str>) -> polodb_core::Result<()> {
        match self {
            Self::Standalone(c) => c.drop_index(name),
            Self::Transaction(c) => c.drop_index(name)
        }
    }

    fn drop(&self) -> polodb_core::Result<()> {
        match self {
            Self::Standalone(c) => c.drop(),
            Self::Transaction(c) => c.drop()
        }
    }

    fn insert_one(&self, doc: impl std::borrow::Borrow<Document>) -> polodb_core::Result<polodb_core::results::InsertOneResult>
    where Document: Serialize {
        match self {
            Self::Standalone(c) => c.insert_one(doc),
            Self::Transaction(c) => c.insert_one(doc)
        }
    }

    fn insert_many(&self, docs: impl IntoIterator<Item = impl std::borrow::Borrow<Document>>) -> polodb_core::Result<polodb_core::results::InsertManyResult>
    where Document: Serialize {
        match self {
            Self::Standalone(c) => c.insert_many(docs),
            Self::Transaction(c) => c.insert_many(docs)
        }
    }

    fn find(&self, filter: Document) -> polodb_core::action::Find<'_, '_, Document>
    where Document: DeserializeOwned + Send + Sync {
        match self {
            Self::Standalone(c) => c.find(filter),
            Self::Transaction(c) => c.find(filter)
        }
    }

    fn find_one(&self, filter: Document) -> polodb_core::Result<Option<Document>>
    where Document: DeserializeOwned + Send + Sync {
        match self {
            Self::Standalone(c) => c.find_one(filter),
            Self::Transaction(c) => c.find_one(filter)
        }
    }

    fn aggregate(&self, pipeline: impl IntoIterator<Item = Document>) -> polodb_core::action::Aggregate<'_, '_> {
        match self {
            Self::Standalone(c) => c.aggregate(pipeline),
            Self::Transaction(c) => c.aggregate(pipeline)
        }
    }
}

pub struct Collection<T: Serialize + DeserializeOwned + Send + Sync, R: Runtime> {
    database: Database<R>,
    name: String,
    transaction_id: Option<bson::Uuid>,
    _doctype: PhantomData<T>
}

impl<T: Serialize + DeserializeOwned + Send + Sync, R: Runtime> Clone for Collection<T, R> {
    fn clone(&self) -> Self {
        Self {
            database: self.database.clone(),
            name: self.name.clone(),
            transaction_id: self.transaction_id.clone(),
            _doctype: PhantomData
        }
    }
}

impl<T: Serialize + DeserializeOwned + Send + Sync, R: Runtime> Collection<T, R> {
    pub(crate) fn create(db: Database<R>, name: String, transaction_id: Option<bson::Uuid>) -> Self {
        Self {
            database: db.clone(),
            name,
            transaction_id,
            _doctype: PhantomData
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub(crate) async fn collection(&self) -> crate::Result<CollectionType> {
        let db = self.database.db().await?;
        if let Some(id) = self.transaction_id {
            let dbcon = self.database.db_context().await?;
            let transactions = dbcon.transactions.lock().await;
            if let Some(transaction) = transactions.get(&id) {
                Ok(CollectionType::Transaction(transaction.lock().await.collection::<Document>(&self.name())))
            } else {
                Err(crate::Error::unknown_transaction(id.to_string()))
            }
        } else {
            Ok(CollectionType::Standalone(db.lock().await.collection::<Document>(&self.name())))
        }
    }

    pub async fn count_documents(&self) -> crate::Result<u64> {
        self.collection().await?.count_documents().or_else(|e| Err(crate::Error::from(e)))
    }

    pub async fn update_one(&self, query: Document, update: Document) -> crate::Result<UpdateResult> {
        self.collection().await?.update_one(query, update).or_else(|e| Err(crate::Error::from(e)))
    }

    pub async fn update_one_with_options(
        &self,
        query: Document,
        update: Document,
        options: UpdateOptions,
    ) -> crate::Result<UpdateResult> {
        self.collection().await?.update_one_with_options(query, update, options).or_else(|e| Err(crate::Error::from(e)))
    }

    pub async fn update_many(&self, query: Document, update: Document) -> crate::Result<UpdateResult> {
        self.collection().await?.update_many(query, update).or_else(|e| Err(crate::Error::from(e)))
    }

    pub async fn update_many_with_options(
        &self,
        query: Document,
        update: Document,
        options: UpdateOptions,
    ) -> crate::Result<UpdateResult> {
        self.collection().await?.update_many_with_options(query, update, options).or_else(|e| Err(crate::Error::from(e)))
    }

    pub async fn delete_one(&self, query: Document) -> crate::Result<DeleteResult> {
        self.collection().await?.delete_one(query).or_else(|e| Err(crate::Error::from(e)))
    }

    pub async fn delete_many(&self, query: Document) -> crate::Result<DeleteResult> {
        self.collection().await?.delete_many(query).or_else(|e| Err(crate::Error::from(e)))
    }

    pub async fn create_index(&self, index: IndexModel) -> crate::Result<()> {
        self.collection().await?.create_index(index).or_else(|e| Err(crate::Error::from(e)))
    }

    pub async fn drop_index(&self, name: impl AsRef<str>) -> crate::Result<()> {
        self.collection().await?.drop_index(name).or_else(|e| Err(crate::Error::from(e)))
    }

    pub async fn drop(&self) -> crate::Result<()> {
        self.collection().await?.drop().or_else(|e| Err(crate::Error::from(e)))
    }

    pub async fn insert_one(&self, doc: impl Borrow<T>) -> crate::Result<InsertOneResult> {
        self.collection().await?.insert_one(bson::to_document(doc.borrow()).or_else(|e| Err(crate::Error::from(e)))?).or_else(|e| Err(crate::Error::from(e)))
    }

    pub async fn insert_many(
        &self,
        docs: impl IntoIterator<Item = impl Borrow<T>>,
    ) -> crate::Result<InsertManyResult> {
        let mut serialized: Vec<Document> = Vec::new();
        for doc in docs {
            serialized.push(bson::to_document(doc.borrow()).or_else(|e| Err(crate::Error::from(e)))?);
        }

        self.collection().await?.insert_many(serialized).or_else(|e| Err(crate::Error::from(e)))
    }

    pub async fn find(&self, filter: Document, skip: Option<u64>, limit: Option<u64>, sort: Option<Document>) -> crate::Result<Vec<T>> {
        let mut results: Vec<T> = Vec::new();
        let collection = self.collection().await?;
        let mut find = collection.find(filter);
        if let Some(_skip) = skip {
            find = find.skip(_skip);
        }

        if let Some(_limit) = limit {
            find = find.limit(_limit);
        }

        if let Some(_sort) = sort {
            find = find.sort(_sort);
        }

        let docs: Vec<Result<Document, polodb_core::Error>> = find.run().or_else(|e| Err(crate::Error::from(e)))?.collect();
        for dresult in docs {
            results.push(match dresult {
                Ok(doc) => bson::from_document::<T>(doc).or_else(|e| Err(crate::Error::from(e))),
                Err(e) => Err(crate::Error::from(e))
            }?);
        }

        Ok(results)
    }

    pub async fn find_one(&self, filter: Document) -> crate::Result<Option<T>> {
        let raw = self.collection().await?.find_one(filter).or_else(|e| Err(crate::Error::from(e)))?;
        if let Some(doc) = raw {
            Ok(Some(bson::from_document::<T>(doc).or_else(|e| Err(crate::Error::from(e)))?))
        } else {
            Ok(None)
        }
    }
}
