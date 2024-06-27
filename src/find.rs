use std::fs;
use std::path::Path;

fn _get_root() -> String {
    let _root = if cfg!(any(target_os = "unix", target_os = "linux", target_os = "macos")) {
        "/".to_string()
    } else {
        "C:\\".to_string()
    };
    _root
}

fn _find_file(name: &str) -> Option<String> {
    let _root = _get_root();
    _find_file_recursive(Path::new(&_root), name)
}

fn _find_file_recursive(path: &Path, name: &str) -> Option<String> {
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                if let Some(result) = _find_file_recursive(&path, name) {
                    return Some(result);
                }
            } else if path.is_file() && path.file_name().unwrap() == name {
                return Some(path.to_str().unwrap().to_string());
            }
        }
    }
    None
}

fn _find_dir(name: &str) -> Option<String> {
    let _root = _get_root();
    _find_dir_recursive(Path::new(&_root), name)
}

fn _find_dir_recursive(path: &Path, name: &str) -> Option<String> {
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                if path.file_name().unwrap() == name {
                    return Some(path.to_str().unwrap().to_string());
                } else if let Some(result) = _find_dir_recursive(&path, name) {
                    return Some(result);
                }
            }
        }
    }
    None
}

pub fn get_root() -> String {
    let _result = _get_root();
    _result
}

pub fn find_file(name: &str) -> Option<String> {
    let _result = _find_file(name);
    _result
}

pub fn find_dir(name: &str) -> Option<String> {
    let _result = _find_dir(name);
    _result
}

pub fn find_any(name: &str, if_dir: bool) -> Option<String> {
    let _result = if if_dir {
        _find_dir(name)
    } else {
        _find_file(name)
    };
    _result
}