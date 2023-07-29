use crate::tree::node::Node;

pub fn printer(tree: &Node, _list: bool, _all: bool, _rec: bool) {
    println!("{}", regular_fmt(tree));
}

fn regular_fmt(tree: &Node) -> String {
    let names: Vec<&str> = tree.children.iter().map(|c| c.file.as_str()).collect::<Vec<&str>>();
    let size = names.iter().map(|n| n.len()).max().unwrap();
    let mut result: String = String::from("");
    for name in names.iter() {
        println!("{}", size - name.len());
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
