use std::convert::Infallible;

use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> Result<Persistence<R>, Infallible> {
  Ok(Persistence(app.clone()))
}

/// Access to the persistence APIs.
pub struct Persistence<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Persistence<R> {
  
}
