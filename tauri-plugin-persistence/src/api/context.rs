use std::{collections::HashMap, ops::Deref, path::PathBuf, str::FromStr, sync::Arc};

use tauri::{AppHandle, Manager, Runtime};
use tokio::{fs::OpenOptions, sync::Mutex};

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
            .join(&resolved)
            .canonicalize()
            .or(Err(crate::Error::invalid_path(path.as_ref())))?;
        if !joined.starts_with(resolved) {
            return Err(crate::Error::path_escapes_context(path.as_ref()));
        }

        Ok(joined)
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
        self.state().await.databases
    }

    pub(crate) async fn files(&self) -> Arc<Mutex<HashMap<bson::Uuid, ContextFileHandle>>> {
        self.state().await.files
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
                            cursors: Arc::new(Mutex::new(HashMap::new())),
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
                        cursors: Arc::new(Mutex::new(HashMap::new())),
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
            handle: Arc::new(Mutex::new(file)),
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

#[derive(Clone)]
pub struct Database<R: Runtime> {
    context: Context<R>,
    name: String,
    path: String,
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

    pub async fn close(self) -> crate::Result<()> {
        self.context.close_database(self.name()).await
    }
}

#[derive(Clone)]
pub struct FileHandle<R: Runtime> {
    context: Context<R>,
    id: bson::Uuid,
    path: String,
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
}
