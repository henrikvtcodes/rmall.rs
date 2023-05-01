use std::fs;
use std::env;
use std::path::{PathBuf, Path};


fn main() {
    for arg in env::args().skip(1) {
        let path_raw = PathBuf::from(arg);
        let real_path = path_raw.canonicalize().expect("Invalid Path");
        let file_metadata = fs::metadata(&real_path).expect("Error with Metadata");
        if file_metadata.is_dir() {
            if !has_nested_folder(&real_path) {
                fs::remove_dir(real_path).expect("Error deleting directory");
            } else {
                println!("Directory {:?} has nested contents. Delete cancelled.", real_path.as_path().to_string_lossy());
            }
        } else if file_metadata.is_file() {
            fs::remove_file(real_path).expect("Error deleting file");
        }
    }
}

fn has_nested_folder(path: &Path) -> bool {
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