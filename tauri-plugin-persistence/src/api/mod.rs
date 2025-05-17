mod error;
pub mod state;
mod context;

/// Exports a reference to various utility types.
pub mod types;

pub use state::{ContextDB, ContextFileHandle, ContextState, FileHandleMode};
pub use error::{Error, Result};
pub use context::{Context, FileHandle, Database, Collection, Transaction};
