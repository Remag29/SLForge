use std::io::{Error};
use std::os::unix::fs;
use std::path::Path;
use serde::Deserialize;
use crate::files::backup_file;

#[derive(Deserialize)]
pub struct Symlink {
    source: Path,
    destination: Path,
    comment: String,
}

pub fn create_symlinks(symlinks: Vec<Symlink>) -> Result<Vec<String>, Error> {
    let mut results: Vec<Some(String)> = Vec::new();

    for symlink in symlinks {
        let src_path = Path::new(&symlink.source);
        let dst_path = Path::new(&symlink.destination);

        if src_path.exists() {
            if !src_path.is_symlink() {
                backup_file(src_path)?;
            }
        }

        fs::symlink(src_path, dst_path)?;
        results.push(format!("Symlink created from {} to {}", src_path.display(), dst_path.display()));
    }

    Ok(results)
}