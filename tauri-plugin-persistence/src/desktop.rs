use std::{collections::HashMap, str::FromStr, sync::Arc};

use serde::{de::DeserializeOwned, Serialize};
use tauri::{plugin::PluginApi, AppHandle, Manager, Runtime, State};
use tokio::sync::Mutex;

use crate::{api::types::{CollectionSpecifier, ContextSpecifier, DatabaseSpecifier, FileHandleSpecifier}, state::{ContextState, PluginState}};

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Persistence<R>> {
  Ok(Persistence(app.clone()))
}

/// Access to the persistence APIs.
pub struct Persistence<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Persistence<R> {
    fn contexts(&self) -> State<'_, PluginState> {
        self.0.state::<PluginState>()
    }

    fn handle(&self) -> AppHandle<R> {
        self.0.clone()
    }

    pub async fn open_context(&self, name: impl AsRef<str>, path: impl AsRef<str>) -> crate::Result<crate::Context<R>> {
        let ctx = self.contexts();
        let resolved_path = std::path::PathBuf::from_str(path.as_ref()).or(Err(crate::Error::invalid_path(path.as_ref())))?;

        if let Some(ctx) = self.contexts().lock().await.get(&name.as_ref().to_string()) {
            if ctx.root_path == path.as_ref().to_string() {
                Ok(crate::Context::<R>::create(self.handle(), name.as_ref().to_string(), path.as_ref().to_string()))
            } else {
                Err(crate::Error::open_context(name, path, "Context already open at a different path."))
            }
        } else {
            if resolved_path.exists() {
                if resolved_path.is_dir() {
                    let mut contexts = ctx.lock().await;
                    let _ = contexts.insert(name.as_ref().to_string(), ContextState {name: name.as_ref().to_string(), root_path: path.as_ref().to_string(), databases: Arc::new(Mutex::new(HashMap::new())), files: Arc::new(Mutex::new(HashMap::new()))});
                    Ok(crate::Context::<R>::create(self.handle(), name.as_ref().to_string(), path.as_ref().to_string()))
                } else {
                    Err(crate::Error::open_context(name, path, "Specified path is not a directory."))
                }
            } else {
                tokio::fs::create_dir_all(resolved_path).await.or_else(|e| Err(crate::Error::open_context(name.as_ref(), path.as_ref(), format!("Failed to create context directory: {e:?}"))))?;
                let mut contexts = ctx.lock().await;
                let _ = contexts.insert(name.as_ref().to_string(), ContextState {name: name.as_ref().to_string(), root_path: path.as_ref().to_string(), databases: Arc::new(Mutex::new(HashMap::new())), files: Arc::new(Mutex::new(HashMap::new()))});
                Ok(crate::Context::<R>::create(self.handle(), name.as_ref().to_string(), path.as_ref().to_string()))
            }
        }
    }

    pub async fn aliased_context(&self, name: impl AsRef<str>) -> crate::Result<crate::Context<R>> {
        if let Some(ctx) = self.contexts().lock().await.get(&name.as_ref().to_string()) {
            Ok(crate::Context::<R>::create(self.handle(), name.as_ref().to_string(), ctx.root_path.clone()))
        } else {
            Err(crate::Error::unknown_context(name))
        }
    }

    pub async fn context(&self, context: ContextSpecifier) -> crate::Result<crate::Context<R>> {
        match context {
            ContextSpecifier::Aliased { alias } => self.aliased_context(alias).await,
            ContextSpecifier::Direct { alias, path } => self.open_context(alias, path).await
        }
    }

    pub async fn database(&self, context: ContextSpecifier, database: DatabaseSpecifier) -> crate::Result<crate::Database<R>> {
        let context = self.context(context).await?;
        match database {
            DatabaseSpecifier::Aliased { alias } => context.database(alias).await,
            DatabaseSpecifier::Direct { alias, path } => context.open_database(alias, path).await
        }
    }

    pub async fn file_handle(&self, context: ContextSpecifier, file_handle: FileHandleSpecifier) -> crate::Result<crate::FileHandle<R>> {
        let context = self.context(context).await?;
        match file_handle {
            FileHandleSpecifier::Aliased { id } => context.file_handle(id).await,
            FileHandleSpecifier::Direct { path, mode } => context.open_file_handle(path, mode).await
        }
    }

    pub async fn transaction(&self, context: ContextSpecifier, database: DatabaseSpecifier, transaction: bson::Uuid) -> crate::Result<crate::Transaction<R>> {
        let database = self.database(context, database).await?;
        database.get_transaction(transaction).await
    }

    pub async fn collection<T: Serialize + DeserializeOwned + Sync + Send>(&self, context: ContextSpecifier, database: DatabaseSpecifier, collection: CollectionSpecifier) -> crate::Result<crate::Collection<T, R>> {
        let database = self.database(context, database).await?;
        match collection {
            CollectionSpecifier::Global { name } => Ok(database.collection(name).await),
            CollectionSpecifier::Transaction { transaction, name } => {
                let trn = database.get_transaction(transaction).await?;
                Ok(trn.collection(name))
            }
        }
    }
}
