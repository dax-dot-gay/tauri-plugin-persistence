use std::{collections::HashMap, sync::Arc};

use polodb_core::{Database, Transaction};
use serde::{Deserialize, Serialize};
use tokio::{fs::{File, OpenOptions}, sync::Mutex};

#[derive(Clone)]
pub struct ContextDB {
    pub name: String,
    pub path: String,
    pub database: Arc<Mutex<Database>>,
    pub transactions: Arc<Mutex<HashMap<bson::Uuid, Arc<Mutex<Transaction>>>>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "mode")]
pub enum FileHandleMode {
    Create {
        new: bool,
        overwrite: bool
    },
    Write {
        overwrite: bool
    },
    Read {}
}

impl FileHandleMode {
    pub fn create_new(overwrite: bool) -> Self {
        Self::Create { new: true, overwrite }
    }

    pub fn create_or_open(overwrite: bool) -> Self {
        Self::Create { new: false, overwrite }
    }

    pub fn append() -> Self {
        Self::Write { overwrite: false }
    }

    pub fn overwrite() -> Self {
        Self::Write { overwrite: true }
    }

    pub fn read() -> Self {
        Self::Read {  }
    }

    pub fn create(&self) -> bool {
        if let Self::Create { .. } = self {
            true
        } else {
            false
        }
    }

    pub fn writeable(&self) -> bool {
        match self {
            Self::Create {..} | Self::Write {..} => true,
            _ => false
        }
    }

    pub fn readable(&self) -> bool {
        match self {
            Self::Read {} => true,
            _ => false
        }
    }
}

impl Into<OpenOptions> for FileHandleMode {
    fn into(self) -> OpenOptions {
        let mut base = OpenOptions::new();
        match self {
            Self::Create { new, overwrite: true } => base.create(true).write(true).create_new(new),
            Self::Create {new, overwrite: false} => base.create(true).append(true).create_new(new),
            Self::Write { overwrite: true } => base.write(true),
            Self::Write { overwrite: false } => base.append(true),
            Self::Read {  } => base.read(true)
        }.clone()
    }
}

#[derive(Clone)]
pub struct ContextFileHandle {
    pub id: bson::Uuid,
    pub path: String,
    pub handle: async_dup::Arc<async_dup::Mutex<File>>,
    pub mode: FileHandleMode
}

#[derive(Clone)]
pub struct ContextState {
    pub name: String,
    pub root_path: String,
    pub databases: Arc<Mutex<HashMap<String, ContextDB>>>,
    pub files: Arc<Mutex<HashMap<bson::Uuid, ContextFileHandle>>>,
}

pub type PluginState = Mutex<HashMap<String, ContextState>>;
