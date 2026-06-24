use std::path::PathBuf;

#[derive(Clone)]
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

    pub fn draw(&self) {
        draw_tree(self, 0);
    }
}

#[derive(Clone)]
pub struct FileNode {
    pub name: String,
    pub size: u64,
}

impl FileNode {
    pub fn new(name: String, size: u64) -> Self {
        Self { name, size }
    }
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
