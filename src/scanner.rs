use anyhow::Result;
use std::fs::{Permissions, read_dir};
use std::path::Path;

use crate::errors::PathError;
use crate::node::{DirNode, FileNode};

pub fn read_directory_recursively(path: &Path, depth: i64) -> Result<DirNode, PathError> {
    if !path.is_dir() {
        return Err(PathError::NotADirectory(path.to_path_buf()));
    }

    let metadata = path.metadata();
    let perm: Option<Permissions>;
    match metadata {
        Ok(metadata) => {
            perm = Some(metadata.permissions());
        }
        Err(_) => {
            perm = None;
        }
    }

    let mut node = DirNode::new(path.to_path_buf(), perm);

    for entry in read_dir(path)? {
        // TODO actually check if ok, mentioned in #1
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_file() {
            let file_size = entry.metadata()?.len();
            let file_name = entry.file_name().to_string_lossy().into_owned();

            node.add_file(FileNode::new(file_name, file_size));
            node.total_size += file_size;
        } else {
            if !entry_path.is_symlink() {
                if depth < 0 {
                    let child_node = read_directory_recursively(&entry_path, depth)?;
                    node.total_size += child_node.total_size;
                    node.add_child(child_node);
                } else if depth > 0 {
                    let child_node = read_directory_recursively(&entry_path, depth - 1)?;
                    node.total_size += child_node.total_size;
                    node.add_child(child_node);
                } else {
                    let child_node = DirNode::from(&entry_path);
                    node.add_child(child_node);
                }
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

        let node = read_directory_recursively(path, -1).unwrap();
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

        let node = read_directory_recursively(path, -1).unwrap();
        assert_eq!(node.files.len(), 1);
    }

    #[test]
    fn test_not_a_dir() {
        let tempdir = tempdir().unwrap();

        let filepath = tempdir.path().join("file.txt");
        File::create(&filepath).unwrap();

        let result = read_directory_recursively(&filepath, -1);

        assert!(result.is_err());
    }
}
