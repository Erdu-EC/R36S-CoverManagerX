use std::ffi::{OsString};
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

pub fn get_files(dir: &str, extensions: Option<Vec<OsString>>) -> std::io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(dir)?;

    let mut files = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(extensions) = &extensions {
                if path.extension().is_none() {
                    continue;
                }
                
                let has_extension = extensions.iter().any(|ext| {
                    let mut undotted_ext = OsString::from(".");
                    undotted_ext.push(path.extension().unwrap());
                    undotted_ext.eq(ext)
                });

                if has_extension {
                    files.push(path);
                }
            } else {
                files.push(path);
            }
        } else if path.is_dir() {
            let mut sub_files = get_files(&path.to_string_lossy(), extensions.clone())?;
            files.append(&mut sub_files);
        }
    }

    Ok(files)
}