use std::str::FromStr;

const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

fn main() {}

enum Entry {
    Directory(Directory),
    File(File),
}

impl Entry {
    fn size(&self) -> usize {
        match self {
            Entry::Directory(dir) => dir.size,
            Entry::File(file) => file.size,
        }
    }
}

struct File {
    name: String,
    size: usize,
}

struct Directory {
    name: String,
    size: usize,
    children: Vec<Entry>,
}

impl Directory {
    fn get_dir_mut(&mut self, path: &[String]) -> Option<&mut Directory> {
        let mut entry = Some(self);

        for dir_name in path {
            entry = entry.and_then(|dir| {
                dir.children.iter_mut().find_map(|child| match child {
                    Entry::Directory(dir) if dir.name == *dir_name => Some(dir),
                    _ => None,
                })
            });
        }

        entry
    }

    /// Assert that all directories have the size of their children
    fn check_size(&self) {
        assert_eq!(
            self.size,
            self.children.iter().map(|child| child.size()).sum()
        );

        for child in &self.children {
            match child {
                Entry::Directory(dir) => dir.check_size(),
                Entry::File(_) => {}
            }
        }
    }
}

enum Command {
    Cd,
    Ls,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cd" => Ok(Command::Cd),
            "ls" => Ok(Command::Ls),
            _ => Err(()),
        }
    }
}

enum LsItem {
    Dir(String),
    File(usize, String),
}

impl FromStr for LsItem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once(' ').unwrap();

        match first {
            "dir" => Ok(LsItem::Dir(second.to_string())),
            size => Ok(LsItem::File(size.parse().unwrap(), second.to_string())),
        }
    }
}

enum CommandWithArgs {
    Cd(String),
    Ls(Vec<LsItem>),
}

impl FromStr for CommandWithArgs {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, args) = s.split_once(&[' ', '\n']).unwrap();
        let command = command.parse::<Command>().unwrap();

        match command {
            Command::Cd => Ok(CommandWithArgs::Cd(args.trim().to_string())),
            Command::Ls => Ok(CommandWithArgs::Ls(
                args.lines()
                    .map(|arg| arg.parse::<LsItem>().unwrap())
                    .collect(),
            )),
        }
    }
}

struct Cursor<'a> {
    root: &'a mut Directory,
    path: Vec<String>,
}

impl Cursor<'_> {
    fn apply_command(&mut self, command: CommandWithArgs) {
        match command {
            CommandWithArgs::Cd(path) if path == "/" => self.path.clear(),
            CommandWithArgs::Cd(path) if path == ".." => drop(self.path.pop().unwrap()),
            CommandWithArgs::Cd(path) => self.path.push(path),
            CommandWithArgs::Ls(items) => {
                for item in items {
                    match item {
                        LsItem::Dir(name) => {
                            self.root.get_dir_mut(&self.path).unwrap().children.push(
                                Entry::Directory(Directory {
                                    name,
                                    size: 0,
                                    children: Vec::new(),
                                }),
                            );
                        }
                        LsItem::File(size, name) => {
                            self.root
                                .get_dir_mut(&self.path)
                                .unwrap()
                                .children
                                .push(Entry::File(File { name, size }));

                            self.update_size(size);
                        }
                    }
                }
            }
        }
    }

    fn update_size(&mut self, size: usize) {
        self.root.size += size;

        let mut dir = &mut *self.root;
        for dir_name in &self.path {
            dir = dir
                .children
                .iter_mut()
                .find_map(|child| match child {
                    Entry::Directory(dir) if dir.name == *dir_name => Some(dir),
                    _ => None,
                })
                .unwrap();

            dir.size += size;
        }
    }
}

fn parse(input: &str) -> Directory {
    let commands = input
        .split("$ ")
        .skip(1)
        .map(|command| command.parse::<CommandWithArgs>().unwrap());

    let mut root = Directory {
        name: "/".to_string(),
        size: 0,
        children: Vec::new(),
    };

    let mut cursor = Cursor {
        root: &mut &mut root,
        path: Vec::new(),
    };

    for command in commands {
        cursor.apply_command(command);
    }

    root
}

fn sum_size_dir_at_most_100k(input: &str) -> usize {
    let root = parse(input);

    let mut size = 0;
    recursive_size_at_most_100k(&root, &mut size);

    size
}

fn recursive_size_at_most_100k(dir: &Directory, size: &mut usize) {
    if dir.size <= 100000 {
        *size += dir.size;
    }

    for child in &dir.children {
        match child {
            Entry::Directory(dir) => recursive_size_at_most_100k(dir, size),
            Entry::File(_) => {}
        }
    }
}

fn smallest_dir(input: &str) -> usize {
    let root = parse(input);

    let needed_space = (30000000 + root.size) - 70000000;

    let mut smallest = usize::MAX;
    recursive_smallest(&root, needed_space, &mut smallest);

    smallest
}

fn recursive_smallest(dir: &Directory, needed_space: usize, smallest: &mut usize) {
    if dir.size < needed_space {
        return;
    }
    if dir.size < *smallest {
        *smallest = dir.size;
    }

    for child in &dir.children {
        match child {
            Entry::Directory(dir) => recursive_smallest(dir, needed_space, smallest),
            Entry::File(_) => {}
        }
    }
}

/// Returns the size the root should have
fn total_size(input: &str) -> usize {
    input
        .lines()
        .flat_map(|line| line.split_whitespace().flat_map(|s| s.parse::<usize>()))
        .sum()
}
