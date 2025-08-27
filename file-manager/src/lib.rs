use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// List paths inside a directory.
pub fn list(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut entries = Vec::new();
    for entry in fs::read_dir(path)? {
        entries.push(entry?.path());
    }
    Ok(entries)
}

/// Copy file from `src` to `dest`.
pub fn copy(src: &Path, dest: &Path) -> io::Result<u64> {
    fs::copy(src, dest)
}

/// Move file from `src` to `dest`.
pub fn move_file(src: &Path, dest: &Path) -> io::Result<()> {
    fs::rename(src, dest)
}

/// Delete file or directory at `path`.
pub fn delete(path: &Path) -> io::Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}
