use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct ChangeDirectory(String);

impl From<&str> for ChangeDirectory {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Cd(ChangeDirectory),
    Ls,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Output {
    Dir(String),
    File(u64, String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Line {
    Output(Output),
    Command(Command),
}

#[derive(Default)]
pub struct Directory {
    size: u64,
    children: HashMap<String, Rc<RefCell<Directory>>>,
    parent: Option<Rc<RefCell<Directory>>>,
}

pub fn create_directory(lines: Vec<Line>) -> Directory {
    let root = Rc::new(RefCell::new(Directory::default()));
    let mut cur = root.clone();
    for line in lines {
        match line {
            Line::Output(Output::Dir(name)) => {
                let entry = cur.borrow_mut().children.entry(name).or_default().clone();
                entry.borrow_mut().parent = Some(cur.clone());
            }
            Line::Output(Output::File(size, path)) => {
                let entry = cur.borrow_mut().children.entry(path).or_default().clone();
                entry.borrow_mut().size = size;
                entry.borrow_mut().parent = Some(cur.clone());
            }
            Line::Command(Command::Ls) => {}
            Line::Command(Command::Cd(ChangeDirectory(dest))) => match dest.as_str() {
                "/" => {
                    // set the current directory to the root
                    cur = root.clone();
                }
                ".." => {
                    // get the parent and set the current directory accordingly
                    let parent = cur.borrow().parent.clone().unwrap();
                    cur = parent;
                }
                _ => {
                    // new child directory. going to assume everything is valid :)
                    let child = cur.borrow_mut().children.entry(dest).or_default().clone();
                    cur = child;
                }
            },
        }
    }

    root.take()
}

impl Directory {
    fn is_directory(&self) -> bool {
        self.size == 0 && !self.children.is_empty()
    }
    fn size(&self) -> u64 {
        self.children
            .values()
            .map(|x| x.borrow().size())
            .sum::<u64>()
            + self.size
    }
}

pub fn all_dir_sizes(dir: Directory) -> (u64, Vec<u64>) {
    let mut v = vec![];
    let total = dir.size();
    recurse(Rc::new(RefCell::new(dir)), &mut v);
    (total, v)
}

fn recurse(d: Rc<RefCell<Directory>>, v: &mut Vec<u64>) {
    if d.borrow().is_directory() {
        v.push(d.borrow().size());
        for child in d.borrow().children.values() {
            recurse(child.clone(), v);
        }
    }
}
