use crate::tree::node::Node;
use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use users::{get_user_by_uid, get_group_by_gid};

pub fn printer(tree: &Node, _list: bool, _all: bool, _rec: bool) {
    println!("{}", regular_fmt(tree));
    if let Some(user) = get_user_by_uid(tree.uid) {
        println!("{user:#?}");
    }
    if let Some(group) = get_group_by_gid(tree.uid) {
        println!("{group:#?}");
    }
}

fn regular_fmt(tree: &Node) -> String {
    let names: Vec<&str> = tree
        .children
        .iter()
        .map(|c| c.file.as_str())
        .collect::<Vec<&str>>();
    let size = names.iter().map(|n| n.len()).max().unwrap();
    let mut result: String = String::from("");
    for name in names.iter() {
        let size = if size - name.len() > 0 {
            size - name.len()
        } else {
            2
        };
        let padding = " ".repeat(size);
        result = format!("{result}{name}{padding}");
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
