//! High-level abstractions for project contexts & database access for Tauri applications.
//! 
//! Primary features:
//! - Database creation & management with PoloDB
//! - Management of open file handles
//! - Basic filesystem operations within the context
//! - Automatic prevention of context escapes


use std::collections::HashMap;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

mod api;
mod commands;
#[cfg(desktop)]
mod desktop;

pub use api::{Collection, Context, Database, Error, FileHandle, Result, Transaction, types, ContextDB, ContextFileHandle, ContextState, FileHandleMode};
pub(crate) use api::state;

#[cfg(desktop)]
pub use desktop::Persistence;
use tauri_specta::collect_commands;
use tokio::sync::Mutex;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the persistence APIs.
pub trait PersistenceExt<R: Runtime> {
    /// Gets a reference to the [Persistence] API
    fn persistence(&self) -> &Persistence<R>;
}

impl<R: Runtime, T: Manager<R>> crate::PersistenceExt<R> for T {
    fn persistence(&self) -> &Persistence<R> {
        self.state::<Persistence<R>>().inner()
    }
}

#[doc(hidden)]
fn builder() -> tauri_specta::Builder<tauri::Wry> {
    tauri_specta::Builder::<tauri::Wry>::new()
    .plugin_name("persistence")
    .commands(collect_commands![
        commands::context,
        commands::database,
        commands::file_handle,
        commands::database_get_collections,
        commands::database_close,
        commands::database_start_transaction,
        commands::database_commit_transaction,
        commands::database_rollback_transaction,
        commands::collection_count_documents,
        commands::collection_update_documents,
        commands::collection_delete_documents,
        commands::collection_create_index,
        commands::collection_drop_index,
        commands::collection_drop,
        commands::collection_insert_documents,
        commands::collection_find_many_documents,
        commands::collection_find_one_document,
        commands::file_close,
        commands::file_write_text,
        commands::file_write_bytes,
        commands::file_read_text,
        commands::file_read_bytes,
        commands::get_context_base_path,
        commands::get_absolute_path_to,
        commands::create_directory,
        commands::remove_directory,
        commands::remove_file,
        commands::file_metadata,
        commands::list_directory,
        commands::close_context,
        commands::cleanup
    ])
}

/// Initializes the plugin.
pub fn init() -> TauriPlugin<tauri::Wry> {
    let builder = builder();

    Builder::new("persistence")
        .invoke_handler(builder.invoke_handler())
        .setup(move |app, api| {
            #[cfg(desktop)]
            let persistence = desktop::init(app, api)?;
            app.manage(persistence);
            app.manage::<state::PluginState>(Mutex::new(HashMap::new()));
            builder.mount_events(app);
            Ok(())
        })
        .build()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn export_types() {
        builder()
            .export(
                specta_typescript::Typescript::default()
                    .formatter(specta_typescript::formatter::prettier)
                    .bigint(specta_typescript::BigIntExportBehavior::Number),
                "./guest-js/commands.ts",
            )
            .expect("failed to export specta types");
    }
}
