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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_build_empty_dir_tree() {
        let tempdir = tempdir().unwrap();
        let path = tempdir.path();

        let node = read_directory_recursively(path).unwrap();
        assert_eq!(node.children.len(), 0);
        assert_eq!(node.files.len(), 0);
    }

    #[test]
    fn test_dir_with_files() {
        let tempdir = tempdir().unwrap();
        let path = tempdir.path();

        let filepath = path.join("a.txt");
        let mut tempfile = File::create(filepath).unwrap();
        writeln!(tempfile, "Hello, world!").unwrap();

        let node = read_directory_recursively(path).unwrap();
        assert_eq!(node.files.len(), 1);
    }

    #[test]
    fn test_not_a_dir() {
        let tempdir = tempdir().unwrap();

        let filepath = tempdir.path().join("file.txt");
        File::create(&filepath).unwrap();

        let result = read_directory_recursively(&filepath);

        assert!(result.is_err());
    }
}
