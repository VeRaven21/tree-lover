use std::fs::{self, read_dir};
use std::path::{Path, PathBuf};

use crate::errors::PathError;
use crate::node::{DirNode, FileNode};

pub fn read_directory_recursively(path: &Path, depth: i64) -> Result<DirNode, PathError> {
    if !path.is_dir() {
        return Err(PathError::NotADirectory(path.to_path_buf()));
    }

    if !readable(&path.to_path_buf()) {
        return Err(PathError::PathUnreadable(path.to_path_buf()));
    }

    let perm = path.metadata().ok().map(|m| m.permissions());

    let mut node = DirNode::new(path.to_path_buf(), perm);

    for entry in read_dir(path)? {
        match entry {
            Ok(entry) => {
                let entry_path = entry.path();
                let filetype = entry.file_type().unwrap();
                if filetype.is_file() {
                    let file_size = entry.metadata()?.len();
                    let file_name = entry.file_name().to_string_lossy().into_owned();

                    node.add_file(FileNode::new(file_name, file_size));
                    node.total_size += file_size;
                } else {
                    if filetype.is_dir() {
                        if depth < 0 {
                            if let Ok(child_node) = read_directory_recursively(&entry_path, depth) {
                                node.add_child(child_node);
                            }
                        } else if depth > 0 {
                            if let Ok(child_node) =
                                read_directory_recursively(&entry_path, depth - 1)
                            {
                                node.add_child(child_node);
                            }
                        } else {
                            let child_node = DirNode::from(&entry_path);
                            node.add_child(child_node);
                        }
                    }
                }
            }
            Err(_) => {}
        }
    }
    Ok(node)
}

fn readable(path: &PathBuf) -> bool {
    match fs::read_dir(path) {
        Ok(_) => true,
        Err(_) => false,
    }
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

    #[test]
    fn test_scan_depth() {
        let main_dir = tempdir().unwrap();
        let main_path = main_dir.path().to_path_buf();

        let path = main_path.join("1/2/3/4/5");

        _ = fs::create_dir_all(&path);

        let node = read_directory_recursively(&main_path, 2).unwrap();
        assert_eq!(node.children[0].children[0].children[0].children_num, 0);
    }
}
