use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

use crate::errors::PathError;

mod errors;

#[derive(Clone)]
struct DirNode {
    path: PathBuf,
    total_size: u64,
    children: Vec<DirNode>,
    files: Vec<FileNode>,
}

impl DirNode {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            total_size: 0,
            files: vec![],
            children: vec![],
        }
    }

    fn draw(&self) {
        draw_tree(self, 0);
    }
}

#[derive(Clone)]
struct FileNode {
    name: String,
    size: u64,
}

impl FileNode {
    fn new(name: String, size: u64) -> Self {
        Self { name, size }
    }
}

fn main() -> Result<()> {
    let path = Path::new(".");

    let tree = read_directory_recursively(path)?;

    tree.draw();

    Ok(())
}

fn read_directory_recursively(path: &Path) -> Result<DirNode, PathError> {
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

fn draw_tree(root: &DirNode, depth: usize) {
    let mut pre_symbols = String::from("╠");
    for _ in 0..depth {
        pre_symbols.push('═')
    }

    let dir_name = root
        .path
        .file_name()
        .unwrap_or(root.path.as_os_str())
        .to_string_lossy();
    println!("{}{} - {}", pre_symbols, dir_name, root.total_size);

    for children in root.children.iter() {
        draw_tree(children, depth + 1);
    }

    for file in root.files.iter() {
        println!("{}═{} - {}", pre_symbols, file.name, file.size);
    }
}
