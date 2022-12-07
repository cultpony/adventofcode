use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
    str::FromStr,
};

use color_eyre::Report;
use itertools::Itertools;
use regex::Regex;

use crate::*;

use super::{Reportable, TaskResult};

#[tracing::instrument]
pub async fn part1() -> Result<Reportable> {
    let mut input = read_file_lines("aoc2022/day7.txt").await?;
    let mut fsspec: Vec<FSSpecLine> = Vec::new();
    while let Some(line) = input.next().await {
        let spec = line.parse()?;
        trace!("Got spec line: {spec:?}");
        fsspec.push(spec);
    }

    let mut fs = FilesystemRef::empty_fs();

    for spec in fsspec {
        spec.apply_to(&mut fs)?;
    }

    fs.chtoroot();

    let small_sums = fs
        .dirs_at_most_sized(100000)
        .iter()
        .map(|x| x.total_size())
        .sum();

    Ok(Reportable {
        year: 2022,
        day: 7,
        part: 1.into(),
        result: TaskResult::Usize(small_sums),
    })
}

#[tracing::instrument]
pub async fn part2() -> Result<Reportable> {
    let mut input = read_file_lines("aoc2022/day7.txt").await?;
    let mut fsspec: Vec<FSSpecLine> = Vec::new();
    while let Some(line) = input.next().await {
        let spec = line.parse()?;
        trace!("Got spec line: {spec:?}");
        fsspec.push(spec);
    }

    let mut fs = FilesystemRef::empty_fs();

    for spec in fsspec {
        spec.apply_to(&mut fs)?;
    }

    fs.chtoroot();

    let max_size = 70000000;
    let needed_free = 30000000;
    let used_size = fs.current_dir.upgrade().unwrap().total_size();
    let unused_now = max_size - used_size;
    assert!(unused_now < needed_free);
    let to_be_freed = needed_free - unused_now;

    debug!("Need to free {to_be_freed}");
    let freed_by_delete = fs
        .dirs_bigger_than_sized(to_be_freed)
        .iter()
        .min_by(|a, b| a.total_size().cmp(&b.total_size()))
        .map(|x| x.total_size())
        .unwrap();

    Ok(Reportable {
        year: 2022,
        day: 7,
        part: 2.into(),
        result: TaskResult::Usize(freed_by_delete),
    })
}

#[derive(Debug)]
pub enum FSSpecLine {
    Dir(String),
    File(String, usize),
    ChangeDir(String),
    Listing,
}

impl FSSpecLine {
    fn apply_to(&self, fs_ref: &mut FilesystemRef) -> Result<()> {
        match self {
            FSSpecLine::Dir(dir) => fs_ref.create_dir(dir).context("creating directory"),
            FSSpecLine::File(file, size) => {
                fs_ref.create_file(file, *size).context("creating file")
            }
            FSSpecLine::ChangeDir(dir) => fs_ref.change_dir(dir),
            FSSpecLine::Listing => Ok(()),
        }
    }
}

impl FromStr for FSSpecLine {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cd_regex = Regex::new(r#"\$ cd (?P<dir>.+)"#).unwrap();
        let ls_regex = Regex::new(r#"\$ ls"#).unwrap();
        let file_regex = Regex::new(r#"(?P<size>\d+) (?P<file>.+)"#).unwrap();
        let dir_regex = Regex::new(r#"dir (?P<dir>.+)"#).unwrap();

        if let Some(captures) = cd_regex.captures(s) {
            Ok(Self::ChangeDir(
                captures.name("dir").unwrap().as_str().to_string(),
            ))
        } else if ls_regex.is_match(s) {
            Ok(Self::Listing)
        } else if let Some(captures) = file_regex.captures(s) {
            Ok(Self::File(
                captures.name("file").unwrap().as_str().to_string(),
                captures.name("size").unwrap().as_str().parse()?,
            ))
        } else if let Some(captures) = dir_regex.captures(s) {
            Ok(Self::Dir(
                captures.name("dir").unwrap().as_str().to_string(),
            ))
        } else {
            Err(Report::msg(format!("Unknown input: {s}")))
        }
    }
}

#[derive(Debug)]
pub struct INode {
    name: String,
    parent: Weak<INode>,
    data: INodeData,
}

impl INode {
    pub fn name(self: &Rc<Self>) -> String {
        self.name.clone()
    }
    // Constructs a root INode
    pub fn root() -> Rc<Self> {
        Rc::new_cyclic(|f| Self {
            name: "".to_string(),
            parent: f.clone(),
            data: INodeData::empty_dir(),
        })
    }

    pub fn total_size(&self) -> usize {
        match self.data {
            INodeData::Directory(ref dirlist) => dirlist
                .borrow()
                .iter()
                .map(|(_, value)| value.total_size())
                .sum(),
            INodeData::File(_) => self.data.size(),
        }
    }

    pub fn new_file(name: String, parent: Weak<INode>, size: usize) -> Rc<INode> {
        Rc::new(INode {
            name,
            parent,
            data: INodeData::file(size),
        })
    }
    pub fn new_dir(name: String, parent: Weak<INode>) -> Rc<INode> {
        Rc::new(INode {
            name,
            parent,
            data: INodeData::empty_dir(),
        })
    }
}

#[derive(Debug)]
pub enum INodeData {
    Directory(RefCell<HashMap<String, Rc<INode>>>),
    File(usize),
}

#[derive(Debug, Clone)]
pub struct FilesystemRef {
    root: Rc<INode>,
    current_dir: Weak<INode>,
}

impl FilesystemRef {
    pub fn empty_fs() -> Self {
        let root = INode::root();
        Self {
            current_dir: Rc::downgrade(&root),
            root,
        }
    }

    pub fn chinode(&self, inode: &Rc<INode>) -> Self {
        Self {
            root: self.root.clone(),
            current_dir: Rc::downgrade(inode),
        }
    }

    pub fn chtoroot(&mut self) {
        self.current_dir = Rc::downgrade(&self.root);
    }

    /// Returns only directories under the current directory
    pub fn sub_dirs(&self) -> Vec<Rc<INode>> {
        match self.current_dir.upgrade() {
            None => panic!("Filesystem free'd while in use"),
            Some(inode) => match &inode.data {
                INodeData::Directory(dirlist) => dirlist
                    .borrow()
                    .iter()
                    .filter(|(_, inode)| inode.data.is_dir())
                    .map(|(_, inode)| inode.clone())
                    .collect_vec(),
                INodeData::File(_) => Vec::new(),
            },
        }
    }

    /// Returns only files under the current directory
    pub fn sub_files(&self) -> Vec<Rc<INode>> {
        match self.current_dir.upgrade() {
            None => panic!("Filesystem free'd while in use"),
            Some(inode) => match &inode.data {
                INodeData::Directory(dirlist) => dirlist
                    .borrow()
                    .iter()
                    .filter(|(_, inode)| !inode.data.is_dir())
                    .map(|(_, inode)| inode.clone())
                    .collect_vec(),
                INodeData::File(_) => Vec::new(),
            },
        }
    }

    pub fn sub_inodes(&self) -> Vec<Rc<INode>> {
        match self.current_dir.upgrade() {
            None => panic!("Filesystem free'd while in use"),
            Some(inode) => match &inode.data {
                INodeData::Directory(dirlist) => dirlist
                    .borrow()
                    .iter()
                    .map(|(_, inode)| inode.clone())
                    .collect_vec(),
                INodeData::File(_) => Vec::new(),
            },
        }
    }

    /// Returns a list of all directories under this one that are smaller than the given size.
    /// The directory itself is not returned when it is smaller
    ///
    /// This will recurse into all subdirectories
    ///
    /// Files from subdirectories are returned first
    pub fn dirs_at_most_sized(&self, size_max: usize) -> Vec<Rc<INode>> {
        debug!(
            "Looking for small folders under {size_max} at {}",
            self.current_path()
        );
        let smaller = self.sub_dirs().into_iter().filter(|x| {
            trace!(
                "Checking folder {} at {} if smaller than {size_max}: {}",
                x.name(),
                self.current_path(),
                x.total_size()
            );
            x.total_size() <= size_max
        });
        self.sub_dirs()
            .iter()
            .flat_map(|x| {
                let fs_ref = self.chinode(x);
                fs_ref.dirs_at_most_sized(size_max)
            })
            .chain(smaller)
            .collect_vec()
    }

    /// Returns directories of atleast this size
    pub fn dirs_bigger_than_sized(&self, size_min: usize) -> Vec<Rc<INode>> {
        debug!(
            "Looking for small folders under {size_min} at {}",
            self.current_path()
        );
        let smaller = self.sub_dirs().into_iter().filter(|x| {
            debug!(
                "Checking folder {} at {} if smaller than {size_min}: {}",
                x.name(),
                self.current_path(),
                x.total_size()
            );
            x.total_size() >= size_min
        });
        self.sub_dirs()
            .iter()
            .flat_map(|x| {
                let fs_ref = self.chinode(x);
                fs_ref.dirs_bigger_than_sized(size_min)
            })
            .chain(smaller)
            .collect_vec()
    }

    pub fn current_path(&self) -> String {
        let mut path = Vec::new();
        let mut refp = self.current_dir.clone();
        while refp
            .upgrade()
            .map(|refpc| {
                // the root is it's own parent
                !std::ptr::eq(refpc.parent.as_ptr(), Rc::as_ptr(&refpc))
            })
            .unwrap()
        {
            let refpr = refp.upgrade().unwrap();
            path.push(refpr.name());
            refp = refpr.parent.clone();
        }
        if path.is_empty() {
            "/".to_string()
        } else {
            // push root
            path.push(String::new());
            path.reverse();
            path.join("/")
        }
    }

    pub fn create_dir(&mut self, dir: &str) -> Result<()> {
        assert!(
            match self.current_dir.upgrade() {
                None => panic!("Filesystem free'd while in use"),
                Some(v) => v.data.is_dir(),
            },
            "Must create file in directory"
        );

        trace!("Creating directory {dir} in {:?}", self.current_path());

        match self.current_dir.upgrade() {
            None => panic!("Filesystem free'd while in use"),
            Some(v) => match &v.data {
                INodeData::File(_) => unreachable!(),
                INodeData::Directory(q) => {
                    let mut cdir = q.borrow_mut();
                    if cdir.contains_key(dir) {
                        return Err(Report::msg("file or directory exists"));
                    }
                    match cdir.insert(
                        dir.to_owned(),
                        INode::new_dir(dir.to_owned(), Rc::downgrade(&v)),
                    ) {
                        Some(_) => unreachable!(),
                        None => Ok(()),
                    }
                }
            },
        }
    }
    pub fn create_file(&mut self, file: &str, size: usize) -> Result<()> {
        assert!(
            match self.current_dir.upgrade() {
                None => panic!("Filesystem free'd while in use"),
                Some(v) => v.data.is_dir(),
            },
            "Must create file in directory"
        );

        trace!(
            "Creating file {file} with size {size} in {:?}",
            self.current_path()
        );

        match self.current_dir.upgrade() {
            None => panic!("Filesystem free'd while in use"),
            Some(v) => match &v.data {
                INodeData::File(_) => unreachable!(),
                INodeData::Directory(q) => {
                    let mut dir = q.borrow_mut();
                    if dir.contains_key(file) {
                        return Err(Report::msg("file or directory exists"));
                    }
                    match dir.insert(
                        file.to_owned(),
                        INode::new_file(file.to_owned(), Rc::downgrade(&v), size),
                    ) {
                        Some(_) => unreachable!(),
                        None => Ok(()),
                    }
                }
            },
        }
    }
    pub fn change_dir(&mut self, dir: &str) -> Result<()> {
        trace!("Changing directory {} via {dir}", self.current_path());

        let res = match dir {
            "." => Ok(()),
            ".." => {
                self.current_dir = match self.current_dir.upgrade() {
                    None => panic!("Filesystem free'd while in use"),
                    Some(cd) => cd.parent.clone(),
                };
                Ok(())
            }
            "/" => {
                self.current_dir = Rc::downgrade(&self.root);
                Ok(())
            }
            dir => {
                let cd = match self.current_dir.upgrade() {
                    None => panic!("Filesystem free'd while in use"),
                    Some(cd) => cd,
                };
                assert!(cd.data.is_dir(), "Current directory is file");
                match cd.data {
                    INodeData::Directory(ref v) => match v.borrow().get(dir) {
                        None => Err(Report::msg(format!("directory {dir} does not exist"))),
                        Some(v) => {
                            self.current_dir = Rc::downgrade(v);
                            Ok(())
                        }
                    },
                    INodeData::File(_) => unreachable!(),
                }
            }
        };
        trace!("New current directory: {}", self.current_path());
        res
    }
}

impl INodeData {
    pub fn is_dir(&self) -> bool {
        match self {
            INodeData::Directory(_) => true,
            INodeData::File(_) => false,
        }
    }
    pub fn size(&self) -> usize {
        match self {
            INodeData::Directory(_) => 0,
            INodeData::File(v) => *v,
        }
    }
    pub fn empty_dir() -> Self {
        INodeData::Directory(RefCell::new(HashMap::new()))
    }
    pub fn file(size: usize) -> Self {
        INodeData::File(size)
    }
}
