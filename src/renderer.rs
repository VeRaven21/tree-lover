use crate::node::DirNode;

pub fn draw_tree(root: &DirNode, depth: usize) {
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