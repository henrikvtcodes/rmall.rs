use std::fs;
use std::path::Path;

pub fn delete_dir(path: &Path, recurse: bool) -> bool {
    let contains_folder = has_nested_folder(&path);
    if !recurse && contains_folder {
        return false;
    } else {
        let result = fs::remove_dir_all(path);
        return result.is_ok();
    }
}

pub fn delete_file(path: &Path) -> bool {
    let result = fs::remove_file(path);
    return result.is_ok();
}

pub fn has_nested_folder(path: &Path) -> bool {
    let mut has_nested = false;
    for entry in fs::read_dir(path).expect("Error with read_dir") {
        let entry = entry.expect("Error with entry");
        let path = entry.path();
        if path.is_dir() {
            has_nested = true;
            break;
        }
    }
    has_nested
}
