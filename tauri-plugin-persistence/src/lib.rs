use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Persistence;
#[cfg(mobile)]
use mobile::Persistence;

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
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let persistence = mobile::init(app, api)?;
      #[cfg(desktop)]
      let persistence = desktop::init(app, api)?;
      app.manage(persistence);
      Ok(())
    })
    .build()
}
