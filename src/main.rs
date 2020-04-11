use std::io;
use std::path::{Path, Component};
use std::collections::BTreeMap;
use std::ffi::OsString;

enum Tree {
    Dir(BTreeMap<OsString, Tree>),
    File(OsString),
}

fn print_prefix(is_last: &[bool]) {
    let mut prefixes = Vec::new();
    prefixes.reserve(is_last.len());

    for (i, il) in is_last.iter().rev().enumerate() {
        if i == 0 {
            prefixes.push(if *il { "└── " } else { "├── " });
        } else {
            prefixes.push(if *il { "    " } else { "│   " });
        }
    }
    prefixes.pop();
    prefixes.reverse();
    for prefix in prefixes.iter() {
        print!("{}", prefix);
    }
}

impl Tree {
    fn insert(&mut self, mut components: Vec<Component>, is_dir: bool) {
        if components.is_empty() {
            return;
        }
        if let Self::Dir(map) = self {
            let root = components.pop().unwrap();
            if let Some(subtree) = map.get_mut(root.as_os_str()) {
                subtree.insert(components, is_dir);
            } else if components.is_empty() && !is_dir {
                let file = Tree::File(root.as_os_str().to_owned());
                map.insert(root.as_os_str().to_owned(), file);
            } else {
                let mut subtree = Tree::Dir(BTreeMap::new());
                subtree.insert(components, is_dir);
                map.insert(root.as_os_str().to_owned(), subtree);
            };
        }
    }

    fn print(&self, is_last: &mut Vec<bool>) {
        match self {
            Self::Dir(map) => {
                for (i, (path, subtree)) in map.iter().enumerate() {
                    let mut is_last = is_last.clone();
                    is_last.push(i + 1 == map.len());
                    print_prefix(&is_last);
                    println!("{}", path.to_str().unwrap());
                    if let Self::Dir(_) = subtree {
                        subtree.print(&mut is_last);
                    }
                }
            }
            Self::File(path) => {
                print_prefix(is_last);
                println!("{}", path.to_str().unwrap());
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut tree = Tree::Dir(BTreeMap::new());
    let mut buffer = String::with_capacity(256);
    loop {
        match io::stdin().read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                if let Some(path) = buffer.trim_end().split_whitespace().last() {
                    let path = Path::new(path);
                    tree.insert(path.components().rev().collect(), path.is_dir());
                }
            }
            _ => break,
        };
        buffer.clear();
    }
    let mut is_last = Vec::new();
    tree.print(&mut is_last);
    Ok(())
}
