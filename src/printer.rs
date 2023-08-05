use crate::tree::node::Node;
use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use users::{get_user_by_uid, get_group_by_gid};

pub fn printer(tree: &Node, list: bool, all: bool, rec: bool) {
    if rec {
        return printer_rec(tree, list, all);
    }
    if list {
        let result =  list_fmt(tree, all);
        return println!("{result}");
    }
    let result = regular_fmt(tree, all);
    println!("{result}");
}

fn printer_rec(tree: &Node, list: bool, all: bool) {
    println!("{}:", tree.parent);
    if list {
        let res =  list_fmt(tree, all);
        println!("{res}");
    } else {
        let res = regular_fmt(tree, all);
        println!("{res}");
    }
    for child in &tree.children {
        if child.is_dir && (!child.file.starts_with('.') || all) {
            printer_rec(child, list, all);
        }
    }
}

fn regular_fmt(node: &Node, all: bool) -> String {
    let names: Vec<&str> = node
        .children
        .iter()
        .map(|c| c.file.as_str())
        .collect::<Vec<&str>>();
    let total_size: usize = names.iter().map(|n| n.len() + 2).sum();
    if let Some(max) = names.iter().map(|n| n.len()).max() {
        if let Some(s) = termsize::get() {
            if s.cols as usize <= total_size - max {
                return regular_col_fmt(&names, s.cols as usize, max);
            }
        }
    }
    let mut result: String = String::new();
    for name in &names {
        if !name.starts_with('.') || all {
            let padding = " ".repeat(2);
            result = format!("{result}{name}{padding}");
        }
    }
    result
}

fn regular_col_fmt(names: &Vec<&str>, width: usize, max: usize) -> String {
    let mut result = String::new();
    let col_number = if width / max <= 5 {
       width / max 
    } else {
       width / max - 2 
    };
    let mut cols: Vec<Vec<&str>> = Vec::new();
    let mut max_len: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < names.len() {
        let col_idx = i % col_number;
        if cols.len() <= col_idx {
            cols.push(Vec::new());
        }
        cols[col_idx].push(names[i]);
        i += 1;
    }
    for col in &cols {
        if let Some(m) = col.iter().map(|n| n.len()).max() {
            max_len.push(m);
        }
    }
    i = 0;
    while i < cols[0].len() {
        let mut c = 0;
        while c < cols.len() {
            if i < cols[c].len() {
                let padding = " ".repeat(max_len[c] - cols[c][i].len() + 2);
                result = format!("{result}{}{padding}", cols[c][i]);
            }
            c += 1;
        }
        if i < cols[0].len() - 1 {
            result = format!("{result}\n");
        }
        i += 1;
    }
    result
}

fn list_fmt(node: &Node, all: bool) -> String {
    let mut result = String::new();
    let sizes: Vec<String> = node.children.iter().map(|c| c.size.to_string()).collect();
    if let Some(m) = sizes.iter().map(std::string::String::len).max() {
        for (i, child) in node.children.iter().enumerate() {
            if !child.file.starts_with('.') || all {
                let file_type = type_char(child);
                let permissions = parse_permissions(child.mode);
                let user = get_user_by_uid(child.uid).unwrap().name().to_str().unwrap().to_owned();
                let group = get_group_by_gid(child.gid).unwrap().name().to_str().unwrap().to_owned();
                let padding = " ".repeat(m - child.size.to_string().len());
                if i == node.children.len() - 1 {
                    result = format!("{result}{file_type}{permissions} 1 {user} {group} {padding}{} {} {}", child.size, child.modified.format("%b %_d %H:%M"), child.file);
                } else {
                    result = format!("{result}{file_type}{permissions} 1 {user} {group} {padding}{} {} {}\n", child.size, child.modified.format("%b %_d %H:%M"), child.file);
                }
            }
        }

    }
    result
}

fn type_char(tree: &Node) -> String {
    if tree.is_dir {
        return String::from("d");
    }
    if tree.is_symlink {
        return String::from("l");
    }
    String::from("-")
}

fn parse_permissions(mode: u32) -> String {
    let user = triplet(mode, S_IRUSR, S_IWUSR, S_IXUSR);
	let group = triplet(mode, S_IRGRP, S_IWGRP, S_IXGRP);
	let other = triplet(mode, S_IROTH, S_IWOTH, S_IXOTH);
	[user, group, other].join("")
}

fn triplet(mode: u32, read: u32, write: u32, execute: u32) -> String {
	match (mode & read, mode & write, mode & execute) {
		(0, 0, 0) => "---",
		(_, 0, 0) => "r--",
		(0, _, 0) => "-w-",
		(0, 0, _) => "--x",
		(_, 0, _) => "r-x",
		(_, _, 0) => "rw-",
		(0, _, _) => "-wx",
		(_, _, _) => "rwx",
	}.to_string()
}
