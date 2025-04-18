mod error;
pub(crate) mod state;
mod context;

/// Exports a reference to various utility types.
pub mod types;

pub use error::{Error, Result};
pub use context::{Context, FileHandle, Database, Collection, Transaction};
