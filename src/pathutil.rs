use std::path::{Path, PathBuf};

pub fn parse_path(input: &String) -> Result<PathBuf, &str> {
    let raw_path = Path::new(input);

    if raw_path.canonicalize().is_ok() {
        let path = raw_path.canonicalize().unwrap();
        Ok(path)
    } else {
        Err("Error canonicalizing path")
    }
}
