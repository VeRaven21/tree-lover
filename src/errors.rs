use std;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PathError {
    // #[error("Path not found: {0}")]
    // NotFound(PathBuf),
    #[error("Path is not a directory: {0}")]
    NotADirectory(PathBuf),

    #[error("IO error for path: {0}")]
    IoError(#[from] std::io::Error),
}
