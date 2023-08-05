#![warn(clippy::all, clippy::pedantic)]

use clap::Parser;
use std::io;

mod tree;
use tree::node::Node;

mod printer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = String::from("."))]
    folder: String,

    /// displays in a list format
    #[arg(short)]
    list: bool,

    /// also displays dot files and folders
    #[arg(short)]
    all: bool,

    /// also displays folders' content recursively
    #[arg(short)]
    recursive: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut tree = Node::new(args.folder.as_str())?;

    tree.parse_dirs(args.recursive)?;
    printer::printer(&tree, args.list, args.all, args.recursive);
    Ok(())
}
