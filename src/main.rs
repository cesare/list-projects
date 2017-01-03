use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn find_root_directory_by_env() -> Option<PathBuf> {
    match env::var("LIST_PROJECTS_DIR") {
        Ok(value) => Some(PathBuf::from(value)),
        Err(_) => None,
    }
}

fn find_root_directory_by_default() -> Option<PathBuf> {
    let home = env::home_dir();
    match home {
        Some(path) => Some(path.join("projects")),
        None => None,
    }
}

fn find_root_directory() -> Result<PathBuf, String> {
    match find_root_directory_by_env() {
        Some(path) => Ok(path),
        None => {
            match find_root_directory_by_default() {
                Some(path) => Ok(path),
                None => Err(String::from("Failed to determine the root directory")),
            }
        }
    }
}

fn filter_directories(root: &Path, to_depth: u32) -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = vec![];

    if to_depth == 0 {
        return dirs;
    }

    match fs::read_dir(root) {
        Ok(current_dir) => {
            for entry in current_dir {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_dir() {
                            let subdirs = filter_directories(&path, to_depth - 1);
                            dirs.push(path);
                            dirs.extend_from_slice(&subdirs);
                        }
                    }
                    Err(_) => (),
                }
            }
            dirs
        }
        Err(_) => dirs,
    }
}

fn list_directories(path: &Path) {
    let directories = filter_directories(path, 2);
    for path in &directories {
        println!("{}", path.display());
    }
}

fn main() {
    match find_root_directory() {
        Ok(root) => list_directories(&root),
        Err(message) => println!("{}", message),
    }
}
