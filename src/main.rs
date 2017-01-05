use std::env;
use std::fs::{self, ReadDir};
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

fn starts_with_dot(path: &PathBuf) -> bool {
    match path.file_name() {
        Some(osstr) => {
            match osstr.to_str() {
                Some(name) => name.starts_with("."),
                None => false,
            }
        }
        None => false,
    }
}

fn path_should_appear(path: &PathBuf) -> bool {
    path.is_dir() && !starts_with_dot(path)
}

fn select_directories(dir: ReadDir) -> Vec<PathBuf> {
    dir.filter_map(|entry| entry.map(|e| e.path()).ok())
        .filter(|path| path_should_appear(&path))
        .collect()
}

fn filter_directories(root: &Path, to_depth: u32) -> Vec<PathBuf> {
    if to_depth == 0 {
        return vec![];
    }

    let paths: Vec<PathBuf> = match fs::read_dir(root) {
        Ok(current_dir) => select_directories(current_dir),
        Err(_) => vec![],
    };

    let mut dirs: Vec<PathBuf> = vec![];

    for path in paths {
        let subdirs = filter_directories(&path, to_depth - 1);
        dirs.push(path);
        dirs.extend_from_slice(&subdirs);
    }
    dirs
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
