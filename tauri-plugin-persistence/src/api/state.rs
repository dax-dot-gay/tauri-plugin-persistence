use std::{collections::HashMap, sync::Arc};

use polodb_core::{ClientCursor, Database, Transaction};
use tokio::{fs::File, sync::Mutex};

#[derive(Clone)]
pub struct ContextDB {
    pub name: String,
    pub path: String,
    pub database: Arc<Mutex<Database>>,
    pub cursors: HashMap<String, Arc<Mutex<ClientCursor<bson::Document>>>>,
    pub transactions: HashMap<String, Arc<Mutex<Transaction>>>,
}

#[derive(Clone)]
pub struct ContextFileHandle {
    pub path: String,
    pub basename: String,
    pub handle: Arc<Mutex<File>>,
}

#[derive(Clone)]
pub struct ContextState {
    pub name: String,
    pub root_path: String,
    pub databases: HashMap<String, ContextDB>,
    pub files: HashMap<String, ContextFileHandle>,
}

pub type PluginState = Arc<Mutex<HashMap<String, ContextState>>>;
