use std::fs;
use std::path::{PathBuf};

pub fn get_dir_entries(dir: &str) -> std::io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(dir)?;
    
    let mut dirs = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            dirs.push(path);
        }
    }

    Ok(dirs)
}