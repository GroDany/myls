use crate::tree::node::Node;
use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use users::{get_user_by_uid, get_group_by_gid};

pub fn printer(tree: &Node, _list: bool, _all: bool, _rec: bool) {
    let res = regular_fmt(tree);
    println!("{}", res);
    // if let Some(user) = get_user_by_uid(tree.uid) {
    //     println!("{user:#?}");
    // }
    // if let Some(group) = get_group_by_gid(tree.uid) {
    //     println!("{group:#?}");
    // }
}

fn regular_fmt(node: &Node) -> String {
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
    let mut result: String = String::from("");
    for name in names.iter() {
        let padding = " ".repeat(2);
        result = format!("{result}{name}{padding}");
    }
    result
}

fn regular_col_fmt(names: &Vec<&str>, width: usize, max: usize) -> String {
    let mut result = String::new();
    let col_number = width / max;
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
    for col in cols.iter() {
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
    //dbg!(&result);
    result
}

fn list_fmt(node: &Node) -> String {
    String::new()
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
