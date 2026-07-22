use crate::filesystem::errors::FilesystemError;
use crate::filesystem::node::{Node, NodeType};

use std::fs::{remove_dir_all, remove_file};
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct FileSystem {
    pub arena: Vec<Node>,
}

impl FileSystem {
    /// Add new node to filesystem. Returns index of added node
    #[must_use]
    pub fn add_node(&mut self, node: Node) -> usize {
        let l = self.arena.len();
        if let Some(parent_index) = node.parent {
            self.arena[parent_index].children.push(l);
            self.add_size(parent_index, node.size);
            self.arena.push(node);
            self.sort_node_children(parent_index);
        } else {
            self.arena.push(node);
        }
        l
    }

    pub fn delete_node(&mut self, index: usize) -> Result<(), FilesystemError> {
        let size: usize = self.arena[index].size;
        let node_type: NodeType = self.arena[index].node_type;
        let path: PathBuf = self.arena[index].path.clone();

        match node_type {
            NodeType::File => {
                if remove_file(&path).is_err() {
                    return Err(FilesystemError::EntryDeleteError(path));
                }

                self.arena[index].mark_deleted();

                if let Some(parent_index) = self.arena[index].parent {
                    self.substract_size(parent_index, size);
                }
            }

            NodeType::Dir => {
                if remove_dir_all(&path).is_err() {
                    return Err(FilesystemError::EntryDeleteError(path));
                }

                self.arena[index].mark_deleted();

                if let Some(parent_index) = self.arena[index].parent {
                    self.substract_size(parent_index, size);
                }
            }
            _ => {
                todo!()
            }
        }

        Ok(())
    }

    /// add size to all containing nodes
    fn add_size(&mut self, node_index: usize, size: usize) {
        self.arena[node_index].size += size;
        let mut cursor = node_index;
        while let Some(parent_index) = self.arena[cursor].parent {
            self.arena[parent_index].size += size;
            cursor = parent_index;
        }
    }
    fn substract_size(&mut self, node_index: usize, size: usize) {
        self.arena[node_index].size -= size;
        let mut cursor = node_index;
        while let Some(parent_index) = self.arena[cursor].parent {
            self.arena[parent_index].size -= size;
            cursor = parent_index;
        }
    }

    pub fn node_children(&self, node: usize, dots: bool) -> Vec<usize> {
        let mut r: Vec<usize> = vec![];
        for &index in self.arena[node].children.iter() {
            if dots | !self.arena[index].name().starts_with(".") {
                r.push(index);
            }
        }
        r
    }

    pub fn sort_node_children(&mut self, node_index: usize) {
        let mut sized_children: Vec<(usize, usize)> = self.arena[node_index]
            .children
            .iter()
            .map(|&child_index| (child_index, self.arena[child_index].size))
            .collect();

        sized_children.sort_by_key(|&(_, size)| std::cmp::Reverse(size));

        self.arena[node_index].children =
            sized_children.into_iter().map(|(index, _)| index).collect();
    }
}
