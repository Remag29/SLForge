use std::fs::{File, rename, remove_file};
use std::io::{Error, ErrorKind};
use std::path::Path;
use crate::symlinks::Symlink;


pub fn delete_file(file_path: &Path) -> Result<String, Error> {
    if remove_file(file_path).is_ok() {
        Ok(format!("File {} deleted successfully", file_path))
    } else {
        Err(Error::new(ErrorKind::NotFound, "File not found"))
    }
}

pub fn rename_file(old_path: &Path, new_path: &Path) -> Result<String, Error> {
    if rename(old_path, new_path).is_ok() {
        Ok(format!("File renamed from {} to {}", old_path, new_path))
    } else {
        Err(Error::new(ErrorKind::NotFound, "File not found"))
    }
}

pub fn backup_file(file_path: &Path) -> Result<String, Error> {
    let backup_path = format!("{}.bak", file_path);
    if Path::new(file_path).exists() {
        rename(file_path, &backup_path)?;
        Ok(format!("File {} backed up as {}", file_path, backup_path))
    } else {
        Err(Error::new(ErrorKind::NotFound, "File not found"))
    }
}

pub fn read_symlink_file(file_path: &Path) -> Result<Vec<Symlink>, Error> {
    let file: File = File::open(file_path)?;
    let symlinks: Vec<Symlink> = serde_json::from_reader(file)?;
    Ok(symlinks)
}