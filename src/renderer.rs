use crate::node::DirNode;

pub fn draw_tree(root: &DirNode, depth: usize, print_files: bool) -> String {
    let mut res = String::new();
    res.push_str(&dir_name(root));
    res.push_str(&draw_dirs(&root.children, depth + 1, print_files));
    res
}

fn draw_dirs(dirs: &Vec<DirNode>, depth: usize, files: bool) -> String {
    let n = dirs.len();
    let mut res = String::new();

    for i in 0..n {
        let mut prefix = String::new();
        if depth >= 2 {
            for _ in 0..depth - 1 {
                prefix.push('┃');
            }
        }
        if n == 1 {
            prefix.push('┝');
        } else if i == 0 && n > 1 {
            prefix.push('┢');
        } else if i == n - 1 && !files {
            prefix.push('┗');
        } else if i == n - 1 && files {
            prefix.push('┡');
        }

        let dir_name = dir_name(&dirs[i]);
        if i < n - 1 {
            res.push_str(&format!("{}{}\n", &prefix, dir_name));
        } else {
            res.push_str(&format!("{}{}", &prefix, dir_name));
        }
    }
    res
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

fn dir_name(dir: &DirNode) -> String {
    String::from(
        dir.path
            .file_name()
            .unwrap_or(dir.path.as_os_str())
            .to_string_lossy(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_size_edge_cases() {
        assert_eq!(get_size(0), "0 b");
        assert_eq!(get_size(1), "1 b");
        assert_eq!(get_size(1023), "1023 b");
        assert_eq!(get_size(1024), "1 KB");
        assert_eq!(get_size(1152921504606847000), "1024 PB")
    }
}
