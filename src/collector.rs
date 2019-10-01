use std::error::Error;
use walkdir::{DirEntry, WalkDir};
use crate::repository::*;

pub fn is_repository_mark_directory(entry: &DirEntry) -> bool {
    entry.file_type().is_dir() && entry.file_name().to_str().map(|s| s == ".git").unwrap_or(false)
}

pub fn is_hidden_directory(entry: &DirEntry) -> bool {
    entry.file_type().is_dir() && entry.file_name().to_str().map(|s| s.starts_with(".")).unwrap_or(false)
}

pub fn collect_repositories(root_path: String, repositories: RepositoryStore) -> Result<(), Box<dyn Error>> {
    let mut it = WalkDir::new(root_path).follow_links(true).into_iter();
    loop {
        let entry = match it.next() {
            None => break,
            Some(Err(_)) => continue,
            Some(Ok(entry)) => entry,
        };
        if is_repository_mark_directory(&entry) {
            repositories.add(Repository::new(entry.clone()))?;
        }
        if is_hidden_directory(&entry) {
            it.skip_current_dir();
        }
    }
    Ok(())
}


