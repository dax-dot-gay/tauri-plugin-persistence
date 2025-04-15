mod error;
pub mod state;
mod context;

pub use error::{Error, Result};
pub use context::{Context, FileHandle, Database, Collection, Transaction};
pub(crate) use context::CollectionType;
