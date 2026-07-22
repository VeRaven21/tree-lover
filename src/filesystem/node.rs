use crate::filesystem::errors::NodeError;
use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Node {
    pub path: PathBuf,
    pub node_type: NodeType,
    pub is_deleted: bool,
    pub size: usize,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeType {
    Dir,
    File,
    SymLink, // Probably won't use it rn
}

impl Node {
    /// Create new node. Path is not
    /// corelated with real filesystem paths
    ///
    /// By default, node is  typed as Dir
    /// Change that with `as_file()` or `as_symlink()`
    pub fn new<P: AsRef<Path>>(path: P, metadata: Metadata) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            node_type: NodeType::Dir,
            is_deleted: false,
            size: metadata.size() as usize,
            parent: None,
            children: vec![],
        }
    }

    pub fn with_parent(mut self, parent: usize) -> Self {
        self.parent = Some(parent);
        self
    }

    pub fn file(mut self) -> Self {
        self.node_type = NodeType::File;
        self
    }

    pub fn dir(mut self) -> Self {
        self.node_type = NodeType::Dir;
        self
    }

    pub fn symlink(mut self) -> Self {
        self.node_type = NodeType::SymLink;
        self
    }

    pub fn mark_deleted(&mut self) {
        self.is_deleted = true;
    }

    pub fn add_child(&mut self, child: usize, child_size: usize) -> Result<(), NodeError> {
        match self.node_type {
            NodeType::File | NodeType::SymLink => {
                Err(NodeError::AddToNotDirError(self.path.clone()))
            }
            _ => {
                self.children.push(child);
                self.size += child_size;
                Ok(())
            }
        }
    }

    pub fn delete(&mut self) {
        self.is_deleted = true;
    }

    pub fn substract_size(&mut self, size: usize) {
        self.size -= size;
    }

    /// Returns last part of path as `String` to be used in rendering
    pub fn name(&self) -> String {
        self.path
            .file_name()
            .unwrap_or(self.path.as_os_str())
            .to_string_lossy()
            .into_owned()
    }

    pub fn num_children(&self) -> usize {
        self.children.len()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
