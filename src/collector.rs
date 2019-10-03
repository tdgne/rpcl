use std::error::Error;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};
use crate::repository::*;
use gitignore;

fn is_repository_mark_directory(entry: &DirEntry) -> bool {
    entry.file_type().is_dir() && entry.file_name().to_str().map(|s| s == ".git").unwrap_or(false)
}

fn is_hidden_directory(entry: &DirEntry) -> bool {
    entry.file_type().is_dir() && entry.file_name().to_str().map(|s| s.starts_with(".")).unwrap_or(false)
}

fn calculate_size(root_entry: DirEntry) -> Result<u64, Box<dyn Error>> {
    let mut it = WalkDir::new(root_entry.path()).follow_links(false).into_iter();
    let mut size = 0u64;
    loop {
        let entry = match it.next() {
            None => break,
            Some(Err(_)) => continue,
            Some(Ok(entry)) => entry,
        };
        if entry.path_is_symlink() {
            continue;
        }
        if entry.file_name().to_str().map(|s| s == ".git").unwrap_or(false) {
            continue;
        }
        size += entry.metadata()?.len();
    }
    Ok(size)
}

// TODO: This only looks at the gitignore at the repository root
/// Collects information of paths listed in `.gitignore`.
fn collect_ignored_path_infos(repository_path: PathBuf) -> Result<Vec<IgnoredPathInfo>, Box<dyn Error>> {
    let mut ignored_path_infos = Vec::new();
    let mut gitignore_path = repository_path.clone();
    gitignore_path.push(".gitignore");
    let ignore = gitignore::File::new(gitignore_path.as_path());
    let ignore = match ignore {
        Err(_) => return Ok(ignored_path_infos),
        Ok(ignore) => ignore,
    };
    // I don't want to bother with symlinks within repositories
    let mut it = WalkDir::new(repository_path).follow_links(false).into_iter();
    loop {
        let entry = match it.next() {
            None => break,
            Some(Err(_)) => continue,
            Some(Ok(entry)) => entry,
        };
        if ignore.is_excluded(entry.path())? {
            let size = calculate_size(entry.clone())?;
            ignored_path_infos.push(IgnoredPathInfo::new(entry.clone().into_path(), size));
            it.skip_current_dir();
        }
    }
    Ok(ignored_path_infos)
}

/// Collects all paths that are considered a git repository.
pub fn collect_repositories(root_path: String, repositories: RepositoryStore) -> Result<(), Box<dyn Error>> {
    let mut it = WalkDir::new(root_path).follow_links(true).into_iter();
    loop {
        let entry = match it.next() {
            None => break,
            Some(Err(_)) => continue,
            Some(Ok(entry)) => entry,
        };
        if is_repository_mark_directory(&entry) {
            let repository_path = {
                let mut path = entry.clone().into_path();
                path.pop();
                path
            };
            repositories.add(Repository::new(repository_path.clone(), collect_ignored_path_infos(repository_path)?))?;
        }
        if is_hidden_directory(&entry) {
            it.skip_current_dir();
        }
    }
    Ok(())
}


