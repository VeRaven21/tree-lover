use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

use crate::errors::PathError;

mod errors;

struct DirNode {
    path: PathBuf,
    children: Vec<DirNode>,
    files: Vec<FileNode>,
}

impl DirNode {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            files: vec![],
            children: vec![],
        }
    }

    fn size(&self) -> usize {
        let mut counter: usize = 0;

        for file in self.files.iter() {
            counter += file.size;
        }
        counter
    }
}
struct FileNode {
    name: String,
    size: usize,
}

impl FileNode {
    fn new(name: String, size: usize) -> Self {
        Self { name, size }
    }
}

fn main() -> Result<()> {
    let path = Path::new(".");

    let tree = read_directory_recursively(path)?;

    draw_tree(&tree, 0);

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
            let file_size = fs::metadata(entry_path)?.len() as usize;
            let file_name = entry.file_name().to_string_lossy().into_owned();

            node.files.push(FileNode::new(file_name, file_size));
        } else {
            node.children.push(read_directory_recursively(&entry_path)?);
        }
    }

    Ok(node)
}

fn draw_tree(root: &DirNode, depth: usize) {
    let mut pre_symbols = String::from("");
    if depth > 0 {
        for _ in 0..depth {
            pre_symbols.push(' ')
        }
    }
    pre_symbols.push('╠');
    let dir_name = root
        .path
        .file_name()
        .unwrap_or(root.path.as_os_str())
        .to_string_lossy();
    println!("{}{} - {}", pre_symbols, dir_name, root.size());

    for children in root.children.iter() {
        draw_tree(children, depth + 1);
    }

    for file in root.files.iter() {
        println!("{}{} - {}", pre_symbols, file.name, file.size);
    }
}
