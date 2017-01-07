use std::env;
use std::fs::{self, ReadDir};
use std::path::{Path, PathBuf};

extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

const USAGE: &'static str = "
Usage:
    list_project
    list_project --project-root=<dir>
    list_project --help

Options:
    --project-root=<dir>  List directories under dir.
";

#[derive(Debug, RustcDecodable)]
pub struct Args {
    flag_project_root: Option<String>,
}

fn parse_args() -> Args {
    Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit())
}


fn find_root_directory_by_arg(args: &Args) -> Option<PathBuf> {
    args.flag_project_root.clone().map(|s| PathBuf::from(s))
}

fn find_root_directory_by_env() -> Option<PathBuf> {
    env::var("LIST_PROJECTS_DIR").ok().map(|value| PathBuf::from(value))
}

fn find_root_directory_by_default() -> Option<PathBuf> {
    env::home_dir().map(|path| path.join("projects"))
}

fn find_root_directory(args: &Args) -> Result<PathBuf, String> {
    find_root_directory_by_arg(args)
        .or_else(find_root_directory_by_env)
        .or_else(find_root_directory_by_default)
        .ok_or_else(|| String::from("Failed to determine the root directory"))
}

fn starts_with_dot(path: &PathBuf) -> bool {
    path.file_name()
        .and_then(|osstr| osstr.to_str())
        .map_or(false, |name| name.starts_with("."))
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
    let args: Args = parse_args();
    match find_root_directory(&args) {
        Ok(root) => list_directories(&root),
        Err(message) => println!("{}", message),
    }
}
