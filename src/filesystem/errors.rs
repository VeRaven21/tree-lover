use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NodeError {
    #[error("Can't add children to non-dir node: {0}")]
    AddToNotDirError(PathBuf),
}

#[derive(Debug, Error)]
pub enum FilesystemError {
    #[error("Error deleting entry: {0}")]
    EntryDeleteError(PathBuf),
}
