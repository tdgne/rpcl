use std::error::Error;
use std::sync::{Arc, RwLock};
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct IgnoredPathInfo {
    path: PathBuf,
    size: u64,
}

impl IgnoredPathInfo {
    pub fn new(path: PathBuf, size: u64) -> Self {
        Self {
            path, size
        }
    }
    
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}

impl IgnoredPathInfo {
    fn clean(&mut self) -> Result<(), Box<dyn Error>> {
        std::fs::remove_dir_all(self.path.as_path())?;
        self.size = 0;
        Ok(())
    }
}

#[derive(Clone)]
pub struct Repository {
    path: PathBuf,
    ignored_path_infos: Vec<IgnoredPathInfo>,
}

impl Repository {
    pub fn new(path: PathBuf, ignored_path_infos: Vec<IgnoredPathInfo>) -> Self {
        Self { path, ignored_path_infos }
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn size(&self) -> u64 {
        self.ignored_path_infos.iter().fold(0, |acc, i| acc + i.size())
    }

    pub fn ignored_path_infos(&self) -> &Vec<IgnoredPathInfo> {
        &self.ignored_path_infos
    }

    fn clean_ignored_path(&mut self, ignored_path_info: &IgnoredPathInfo) -> Result<(), Box<dyn Error>> {
        for i in self.ignored_path_infos.iter_mut() {
            if i.path() == ignored_path_info.path() {
                i.clean()?;
            }
        }
        self.ignored_path_infos = self.ignored_path_infos.iter()
            .filter(|i| i.size() != 0)
            .map(|i| i.clone())
            .collect::<Vec<_>>();
        Ok(())
    }
}

#[derive(Clone)]
pub struct RepositoryStore {
    store: Arc<RwLock<Vec<Repository>>>,
}

impl RepositoryStore {
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(Vec::new())),
        }
    }

    //FIXME: return an Error instead of panic!
    pub fn add(&self, repository: Repository) -> Result<(), Box<dyn Error>> {
        self.store.clone().write().expect("RwLock Error").push(repository);
        Ok(())
    }

    //FIXME: return an Error instead of panic!
    pub fn repositories(&self) -> Result<Vec<Repository>, Box<dyn Error>> {
        Ok(self.store.clone().read().expect("RwLock Error").clone())
    }

    pub fn repositories_sorted(&self) -> Result<Vec<Repository>, Box<dyn Error>> {
        let mut repos = self.repositories()?;
        repos.sort_by(|a, b| b.size().cmp(&a.size()));
        Ok(repos)
    }

    pub fn filtered_len(&self) -> Result<usize, Box<dyn Error>> {
        Ok(self.store.clone().read().expect("RwLock Error").iter().filter(|repo| repo.size() > 0).count())
    }

    pub fn find_by_path(&self, path: PathBuf) -> Result<Option<Repository>, Box<dyn Error>> {
        Ok(self.store.clone().read().expect("RwLock Error").iter().find(|r| r.path().to_path_buf() == path).map(|r| r.clone()))
    }

    pub fn clean_ignored_path(&mut self, repository: &Repository, ignored_path_info: &IgnoredPathInfo) -> Result<(), Box<dyn Error>> {
        for r in self.store.clone().write().expect("RwLock Error").iter_mut() {
            if r.path() == repository.path() {
                r.clean_ignored_path(ignored_path_info)?;
            }
        }
        Ok(())
    }
}

