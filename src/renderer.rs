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
    println!("{}{} - {}", pre_symbols, dir_name, get_size(root.total_size));

    for children in root.children.iter() {
        draw_tree(children, depth + 1);
    }

    for file in root.files.iter() {
        println!("{}═{} - {}", pre_symbols, file.name, get_size(file.size));
    }
}


fn get_size(size: u64) -> String {
    if size == 0 {
        return String::from("0 b");
    }

    let postfixes: [&str; 6] = ["b", "KB", "MB", "GB", "TB", "PB"];
    
    let exp = (size.ilog(1024) as usize).min(postfixes.len() - 1);
    
    let divisor = 1024u64.pow(exp as u32);
    let value = size as f64 / divisor as f64;

    if value.fract() == 0.0 {
        format!("{} {}", value as u64, postfixes[exp])
    } else {
        format!("{:.2} {}", value, postfixes[exp])
    }
}