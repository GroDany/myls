use crate::tree::node::Node;
use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use users::{get_user_by_uid, get_group_by_gid};

pub fn printer(tree: &Node, _list: bool, _all: bool, _rec: bool) {
    println!("{}", regular_fmt(tree));
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
    let size: usize = names.iter().map(|n| n.len() + 2).sum();
    if let Some(s) = termsize::get() {
        if s.cols as usize <= size {
            return regular_col_fmt(&names, s.cols as usize);
        }
    }
    let mut result: String = String::from("");
    for name in names.iter() {
        let padding = " ".repeat(2);
        result = format!("{result}{name}{padding}");
    }
    result
}

fn regular_col_fmt(names: &Vec<&str>, width: usize) -> String {
    let mut result = String::from("");
    if let Some(max) = names.iter().map(|n| n.len()).max() {
        // let step = width / max;
        // for name in names.iter() {
        //     let padding = if max - name.len() < 2 {
        //         " ".repeat(2)
        //     } else {
        //         " ".repeat(max - name.len())
        //     };
        //     result = format!("{result}{name}{padding}");
        // }
        
    }
    result
}

fn list_fmt(node: &Node) -> String {
    String::from("")
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
