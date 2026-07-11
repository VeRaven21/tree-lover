use std::fs::{Permissions, metadata};
use std::path::PathBuf;

mod errors;

use errors::DirNodeError;

#[derive(Clone, PartialEq, Debug)]
pub struct DirNode {
    pub path: PathBuf,
    pub total_size: u64,
    pub children: Vec<DirNode>,
    pub children_num: usize,
    pub files: Vec<FileNode>,
    pub files_num: usize,
    pub permissions: Option<Permissions>,
}

impl DirNode {
    pub fn new(path: PathBuf, permissions: Option<Permissions>) -> Self {
        Self {
            path,
            total_size: 0,
            files: vec![],
            files_num: 0,
            children: vec![],
            children_num: 0,
            permissions,
        }
    }

    pub fn num_entries(&self) -> usize {
        self.children.len() + self.files.len()
    }

    pub fn name(&self) -> String {
        self.path
            .file_name()
            .unwrap_or(self.path.as_os_str())
            .to_string_lossy()
            .into_owned()
    }

    pub fn add_child(&mut self, child: DirNode) {
        self.total_size += child.total_size;
        self.children.push(child); // TODO Rewrite to insert in the right place
        self.children
            .sort_by_key(|b| std::cmp::Reverse(b.total_size)); // I think the right way is to add dot folders to the beginning
        self.children_num += 1;
    }

    pub fn add_file(&mut self, file: FileNode) {
        self.total_size += file.size;
        self.files.push(file); // TODO Rewrite to insert in the right place
        self.files.sort_by_key(|b| std::cmp::Reverse(b.size));
        self.files_num += 1;
    }

    pub fn entry(&'_ self, i: usize, dots: bool) -> Result<SomeNode<'_>, DirNodeError> {
        let mut entries: Vec<SomeNode> = vec![];
        for dir in self.children.iter() {
            if dots || !dir.name().starts_with(".") {
                entries.push(SomeNode::Dir(dir));
            }
        }

        for file in self.files.iter() {
            if dots || !file.name.starts_with(".") {
                entries.push(SomeNode::File(file));
            }
        }

        if i >= entries.len() - 1 {
            Err(DirNodeError::IndexOutOfRange(i))
        } else {
            Ok(entries[i])
        }
    }

    pub fn entries(&self) -> Vec<SomeNode<'_>> {
        let mut r: Vec<SomeNode> = vec![];

        for dir in self.children.iter() {
            r.push(SomeNode::Dir(dir));
        }

        for file in self.files.iter() {
            r.push(SomeNode::File(file));
        }

        r
    }
}

impl From<&PathBuf> for DirNode {
    fn from(path: &PathBuf) -> Self {
        // I guess the only difference is that it reads perms itself
        let metadata = metadata(path);
        let perms: Option<Permissions> = match metadata {
            Ok(metadata) => Some(metadata.permissions()),
            Err(_) => None,
        };
        DirNode::new(path.clone(), perms)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct FileNode {
    pub name: String,
    pub size: u64,
}

impl FileNode {
    pub fn new(name: String, size: u64) -> Self {
        Self { name, size }
    }

    pub fn name(&self) -> String {
        self.name.clone() // Made just for consistency. Probably gonna create Node trait just for fun
    }
}

#[derive(Clone, Copy)]
pub enum SomeNode<'a> {
    Dir(&'a DirNode),
    File(&'a FileNode),
}
