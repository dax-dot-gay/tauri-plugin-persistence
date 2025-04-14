use std::{collections::HashMap, sync::Arc};

use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

#[cfg(desktop)]
mod desktop;
mod api;
mod commands;

pub use api::{Error, Result, state};

#[cfg(desktop)]
use desktop::Persistence;
use tokio::sync::Mutex;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the persistence APIs.
pub trait PersistenceExt<R: Runtime> {
  fn persistence(&self) -> &Persistence<R>;
}

impl<R: Runtime, T: Manager<R>> crate::PersistenceExt<R> for T {
  fn persistence(&self) -> &Persistence<R> {
    self.state::<Persistence<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("persistence")
    .invoke_handler(tauri::generate_handler![])
    .setup(|app, api| {
      #[cfg(desktop)]
      let persistence = desktop::init(app, api)?;
      app.manage(persistence);
      app.manage::<state::PluginState>(Arc::new(Mutex::new(HashMap::new())));
      Ok(())
    })
    .build()
}
