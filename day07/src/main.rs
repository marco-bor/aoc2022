use core::panic;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    vec,
};

fn main() {
    let input = include_str!("input.txt");

    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> usize {
    let fs = parse_input(input);
    fs.flat_dirs()
        .iter()
        .filter(|d| d.size() < 100000)
        .map(|d| d.size())
        .sum()
}

fn problem2(input: &str) -> usize {
    let fs = parse_input(input);
    let used_space = fs.size();
    let dirs = fs.flat_dirs();
    let mut dirs = dirs
        .iter()
        .filter(|f| 70000000 - (used_space - f.size()) >= 30000000)
        .map(|d| d.size())
        .collect::<Vec<_>>();
    dirs.sort();
    dirs[0]
}

#[test]
fn test_problem1() {
    assert_eq!(problem1(include_str!("testdata.txt")), 95437);
}

#[test]
fn test_problem2() {
    assert_eq!(problem2(include_str!("testdata.txt")), 24933642);
}

#[derive(Debug)]
enum FS {
    // note: RefCell is used because children are added afterwards
    // and we need the Vec to mutate
    Dir(String, RefCell<Vec<Rc<FS>>>, Option<Weak<FS>>),
    File(String, usize),
}

impl FS {
    /// Return file size or total directory size
    fn size(&self) -> usize {
        match self {
            FS::File(_, size) => *size,
            FS::Dir(_, files, _) => files.borrow().iter().map(|f| f.size()).sum(),
        }
    }

    /// Return file name or directory name
    fn name(&self) -> &String {
        match self {
            FS::File(name, _) => name,
            FS::Dir(name, _, _) => name,
        }
    }

    /// Return child that matches name.
    ///
    /// The function does NOT iterate inner directories
    fn find_child(&self, name: &str) -> Option<Rc<FS>> {
        match self {
            FS::File(..) => None,
            FS::Dir(_, children, _) => children.borrow().iter().find(|f| f.name() == name).cloned(),
        }
    }

    /// Return directory's parent
    fn parent(&self) -> Option<Rc<FS>> {
        match self {
            FS::Dir(_, _, Some(parent)) => parent.upgrade(),
            _ => None,
        }
    }

    /// Push child if self is directory
    fn push_child(&self, child: FS) {
        if let FS::Dir(_, children, _) = self {
            children.borrow_mut().push(Rc::new(child))
        }
    }

    /// Returns `true` if the fs is [`Dir`].
    ///
    /// [`Dir`]: FS::Dir
    #[must_use]
    fn is_dir(&self) -> bool {
        matches!(self, Self::Dir(..))
    }

    /// Return all directories and sub-directories in self
    fn flat_dirs(&self) -> Vec<Rc<FS>> {
        assert!(self.is_dir());

        match self {
            FS::Dir(_, children, _) => children
                .borrow()
                .iter()
                .filter(|x| x.is_dir())
                .flat_map(|x| x.flat_dirs().into_iter().chain(vec![x.clone()]))
                .collect::<Vec<_>>(),
            _ => panic!("cannot flat_dir() a file"),
        }
    }
}

fn parse_input(s: &str) -> Rc<FS> {
    let fs = Rc::new(FS::Dir(String::from("/"), RefCell::new(vec![]), None));

    let mut cwd = fs.clone();

    for line in s.lines() {
        let mut tokens = line.split_whitespace();
        match tokens.next().expect("expected token") {
            "$" => match tokens.next().expect("expected command") {
                "cd" => {
                    cwd = match tokens.next().expect("'cd' requires a second argument") {
                        ".." => cwd.parent().expect("parent does not exist"),
                        "/" => fs.clone(),
                        param => cwd
                            .find_child(param)
                            .unwrap_or_else(|| panic!("folder '{}' does not exist", param)),
                    }
                }
                "ls" => {}
                cmd => panic!("unexpected command '{}'", cmd),
            },
            _ => {
                let (a, name) = line.split_once(' ').expect("invalid output format");

                cwd.push_child(match a {
                    "dir" => FS::Dir(
                        String::from(name),
                        RefCell::new(vec![]),
                        Some(Rc::downgrade(&cwd)),
                    ),
                    _ => FS::File(
                        String::from(name),
                        a.parse::<usize>().expect("invalid file size"),
                    ),
                })
            }
        }
    }
    fs
}

#[test]
fn test_flat_dirs() {
    let fs = parse_input(
        "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d",
    );

    let dirs = fs.flat_dirs();
    let mut dir_names = dirs.iter().map(|d| d.name().to_owned());

    assert_eq!(dir_names.next(), Some("a".to_string()));
    assert_eq!(dir_names.next(), Some("d".to_string()));
}
