use crate::data;
use std::io;
use std::path::PathBuf;
use xxhash_rust::xxh3::xxh3_64;

pub struct HashResult {
    pub action: data::enums::UserAction,
    pub relative_path: String,
    pub value: u64,
}

pub fn xxh3_bytes(bytes: &[u8]) -> u64 {
    xxh3_64(bytes)
}

pub fn size(path: &PathBuf, cwd: &PathBuf) -> io::Result<HashResult> {
    let relative_path = path
        .strip_prefix(cwd)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string();
    let file_length = path.metadata()?.len();
    Ok(HashResult {
        action: data::enums::UserAction::Size,
        relative_path,
        value: file_length,
    })
}

pub fn xxh3(path: &PathBuf, cwd: &PathBuf) -> io::Result<HashResult> {
    let relative_path = path
        .strip_prefix(cwd)
        .unwrap_or(path)
        .to_string_lossy()
        .to_string();
    let file_bytes = std::fs::read(path)?;
    let hash = xxh3_64(&file_bytes);
    Ok(HashResult {
        action: data::enums::UserAction::XXH3,
        relative_path,
        value: hash,
    })
}
