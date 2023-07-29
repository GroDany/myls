// use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
use chrono::{DateTime, Local};
use std::{
    fmt,
    fs::{self},
    io::{self, Error, ErrorKind},
    os::unix::fs::MetadataExt,
    path::Path,
};

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub path: String,
    pub file: String,
    pub parent: String,
    pub is_dir: bool,
    pub is_file: bool,
    pub is_symlink: bool,
    pub size: u64,
    pub mode: u32,
    pub modified: DateTime<Local>,
}

impl Node {
    pub fn new(dir: &str) -> io::Result<Self> {
        let path = Path::new(dir);
        let parent = path
            .parent()
            .ok_or(Error::new(ErrorKind::Other, "Error getting parent folder"))?
            .to_str()
            .ok_or(Error::new(
                ErrorKind::Other,
                "Error getting parent folder to str",
            ))?;
        let mut file = String::from(dir);
        if dir != "." {
            file = String::from(
                path.file_name()
                    .ok_or(Error::new(ErrorKind::Other, "Error getting file name"))?
                    .to_str()
                    .ok_or(Error::new(
                        ErrorKind::Other,
                        "Error getting file name to str",
                    ))?,
            );
        }
        let metadata = path.metadata()?;
        let parent = Some(String::from(parent));
        Ok(Self {
            children: vec![],
            path: String::from(dir),
            file,
            parent: parent.ok_or(Error::new(ErrorKind::Other, "Error parsing parent dir"))?,
            is_dir: path.is_dir(),
            is_file: path.is_file(),
            is_symlink: path.is_symlink(),
            size: metadata.len(),
            mode: metadata.mode(),
            modified: DateTime::from(metadata.modified()?),
        })
    }

    pub fn parse_dirs(&mut self, reccursive: bool, all: bool) -> io::Result<()> {
        let dir = Path::new(self.path.as_str());
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                let name = path
                    .to_str()
                    .ok_or(Error::new(ErrorKind::Other, "Error parsing path"))?;
                // if !name.starts_with(".") || all { 
                let index = self.add_node(name)?;
                if reccursive {
                    self.children[index].parse_dirs(reccursive, all)?;
                }
                //}
            }
        }
        self.children.sort();
        Ok(())
    }

    fn add_node(&mut self, dir: &str) -> io::Result<usize> {
        let node = Self::new(dir);
        self.children.push(node?);
        Ok(self.children.len() - 1)
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.parent.is_empty() {
            write!(f, "{}/", self.parent)?;
        }
        writeln!(f, "{}", self.file)?;
        for child in &self.children {
            child.fmt(f)?;
        }
        Ok(())
    }
}
