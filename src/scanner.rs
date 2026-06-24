use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::errors::PathError;
use crate::node::{DirNode, FileNode};

pub fn read_directory_recursively(path: &Path) -> Result<DirNode, PathError> {
    if !path.is_dir() {
        return Err(PathError::NotADirectory(path.to_path_buf()));
    }

    let mut node = DirNode::new(path.to_path_buf());

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_file() {
            let file_size = entry.metadata()?.len();
            let file_name = entry.file_name().to_string_lossy().into_owned();

            node.files.push(FileNode::new(file_name, file_size));
            node.total_size += file_size;
        } else {
            if !entry_path.is_symlink() {
                let child_node = read_directory_recursively(&entry_path)?;
                node.total_size += child_node.total_size;
                node.children.push(child_node);
            }
        }
    }

    Ok(node)
}
