use std::fs::{self, Metadata, read_dir};
use std::path::{Path, PathBuf};

use crate::errors::PathError;
use crate::filesystem::{FileSystem, Node};

pub fn read_directory_recursively<P: AsRef<Path>>(path: P) -> Result<FileSystem, PathError> {
    let path: PathBuf = path.as_ref().to_path_buf();

    if !path.is_dir() {
        return Err(PathError::NotADirectory(path));
    }

    if !readable(&path) {
        return Err(PathError::PathUnreadable(path));
    }

    let mut filesystem: FileSystem = FileSystem::default();
    let root_metadata: Metadata = path.metadata().unwrap(); // FIXME: Unsafe usage of unwrap, temporary solution
    let root_node: Node = Node::new(path, root_metadata).dir();
    let _root_index = filesystem.add_node(root_node);

    let mut queue: Vec<usize> = vec![]; // Vector of nodes whos children need to be worked on
    queue.push(0); // Push root node

    while let Some(index) = queue.pop() {
        let path = &filesystem.arena[index].path;
        for entry in read_dir(path)?.flatten() {
            let mut flag = false;
            let mut node = Node::new(entry.path(), entry.metadata().unwrap()).with_parent(index);
            let ftype = entry.file_type().unwrap();
            if ftype.is_dir() {
                node = node.dir();
                flag = true;
            } else if ftype.is_file() {
                node = node.file();
            } else if ftype.is_symlink() {
                node = node.symlink();
            }
            let i = filesystem.add_node(node);
            if flag {
                queue.push(i)
            }
        }
    }
    Ok(filesystem)
}

fn readable<P: AsRef<Path>>(path: P) -> bool {
    fs::read_dir(path).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filesystem::node::NodeType;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_build_empty_dir_tree() {
        let tempdir = tempdir().unwrap();
        let path = tempdir.path();

        let fs = read_directory_recursively(path).unwrap();
        assert_eq!(fs.arena.len(), 1);
        assert_eq!(fs.arena[0].children.len(), 0);
    }

    #[test]
    fn test_dir_with_files() {
        let tempdir = tempdir().unwrap();
        let path = tempdir.path();

        let filepath = path.join("a.txt");
        let mut tempfile = File::create(filepath).unwrap();
        writeln!(tempfile, "Hello, world!").unwrap();

        let fs = read_directory_recursively(path).unwrap();
        assert_eq!(fs.arena[0].children.len(), 1);
        assert_eq!(fs.arena[1].node_type, NodeType::File)
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
