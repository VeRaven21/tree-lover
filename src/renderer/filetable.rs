use crate::filesystem::{FileSystem, Node};

use ratatui::layout::Constraint;
use ratatui::widgets::{Row, Table};

pub fn fill_filetable(filesystem: &FileSystem, index: usize, draw_dots: bool) -> Table<'_> {
    let widths: Vec<Constraint> = vec![Constraint::Percentage(60), Constraint::Percentage(40)];
    let mut lines: Vec<[String; 2]> = vec![];

    let node: &Node = &filesystem.arena[index];

    for &child_index in node.children.iter() {
        let child_name = filesystem.arena[child_index].name();
        if draw_dots || !child_name.starts_with(".") {
            lines.push([child_name, get_size(filesystem.arena[child_index].size)]);
        }
    }

    let mut rows: Vec<Row> = vec![];

    for row in lines {
        rows.push(Row::new(row));
    }

    Table::new(rows, widths)
}

fn get_size(size: usize) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_size_edge_cases() {
        assert_eq!(get_size(0), "0 b");
        assert_eq!(get_size(1), "1 b");
        assert_eq!(get_size(1023), "1023 b");
        assert_eq!(get_size(1024), "1 KB");
        assert_eq!(get_size(1099511627776), "1 TB");
    }
}
