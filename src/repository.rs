use std::error::Error;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::Sender;
use walkdir::DirEntry;

#[derive(Clone)]
pub struct Repository {
    dir_entry: DirEntry,
}

impl Repository {
    pub fn new(dir_entry: DirEntry) -> Self {
        Self { dir_entry }
    }
}

pub enum Event {
    Update,
}

#[derive(Clone)]
pub struct RepositoryStore {
    store: Arc<RwLock<Vec<Repository>>>,
    tx: Sender<Event>,
}

impl RepositoryStore {
    pub fn with_sender(tx: Sender<Event>) -> Self {
        Self {
            store: Arc::new(RwLock::new(Vec::new())),
            tx
        }
    }

    //FIXME: return an Error instead of panic!
    pub fn add(&self, repository: Repository) -> Result<(), Box<dyn Error>> {
        self.store.clone().write().expect("RwLock Error").push(repository);
        self.tx.send(Event::Update)?;
        Ok(())
    }

    //FIXME: return an Error instead of panic!
    pub fn repositorys(&self) -> Result<Vec<Repository>, Box<dyn Error>> {
        Ok(self.store.clone().read().expect("RwLock Error").clone())
    }
}

