use std::path::PathBuf;

use crate::renderer::draw_tree;

#[derive(Clone, PartialEq)]
pub struct DirNode {
    pub path: PathBuf,
    pub total_size: u64,
    pub children: Vec<DirNode>,
    pub files: Vec<FileNode>,
}

impl DirNode {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            total_size: 0,
            files: vec![],
            children: vec![],
        }
    }

    pub fn draw(&self, print_files: bool) {
        println!("{}", draw_tree(self, 0, print_files));
    }
}

#[derive(Clone, PartialEq)]
pub struct FileNode {
    pub name: String,
    pub size: u64,
}

impl FileNode {
    pub fn new(name: String, size: u64) -> Self {
        Self { name, size }
    }
}
