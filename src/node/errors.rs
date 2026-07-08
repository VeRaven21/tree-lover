use thiserror::Error;

#[derive(Debug, Error)]
pub enum DirNodeError {
    #[error("Index out of entries range: {0}")]
    IndexOutOfRange(usize),
}
