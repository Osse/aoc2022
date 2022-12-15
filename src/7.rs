use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
enum Line {
    Command(Command),
    Output(Output),
}

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
}

#[derive(Debug)]
enum Output {
    File(usize, String),
    Dir(String),
}

#[derive(Debug)]
enum FsEntry {
    File(usize, String),
    Dir(usize, String),
}

fn parse_line(line: &str) -> Line {
    if let Some(command) = line.strip_prefix("$ ") {
        if let Some(dir) = command.strip_prefix("cd ") {
            Line::Command(Command::Cd(dir.to_owned()))
        } else if command == "ls" {
            Line::Command(Command::Ls)
        } else {
            panic!("wtf command")
        }
    } else if let Some(dir) = line.strip_prefix("dir ") {
        Line::Output(Output::Dir(dir.to_owned()))
    } else {
        let (s, n) = line.split_once(' ').expect("find space");
        Line::Output(Output::File(s.parse().unwrap(), n.to_owned()))
    }
}

fn main() {
    let contents = std::fs::read_to_string("inputs/7.txt").expect("read input");

    let mut file_system = HashMap::<PathBuf, FsEntry>::new();
    let mut cwd = PathBuf::from("/");

    file_system.insert(cwd.clone(), FsEntry::Dir(0, String::from("/")));

    for l in contents.lines().map(parse_line) {
        match l {
            Line::Command(c) => match c {
                Command::Cd(dir) => {
                    if dir.starts_with("/") {
                        cwd = PathBuf::from(&dir);
                    } else if &dir == ".." {
                        cwd.pop();
                    } else {
                        cwd.push(dir);
                    }
                }
                Command::Ls => (),
            },
            Line::Output(o) => match o {
                Output::Dir(dir) => {
                    let mut new_dir = cwd.clone();
                    new_dir.push(&dir);
                    file_system.insert(new_dir, FsEntry::Dir(0, dir));
                }
                Output::File(sz, name) => {
                    let e = file_system.get_mut(&cwd).unwrap();
                    match e {
                        FsEntry::Dir(ref mut dir_size, _) => {
                            *dir_size += sz;
                        }
                        _ => panic!("wtf"),
                    };

                    let mut itercwd = cwd.clone();

                    while itercwd.pop() {
                        let e = file_system.get_mut(&itercwd).unwrap();
                        match e {
                            FsEntry::Dir(ref mut dir_size, _) => {
                                *dir_size += sz;
                            }
                            _ => panic!("wtf"),
                        };
                    }

                    let mut new_file = cwd.clone();
                    new_file.push(&name);
                    file_system.insert(new_file, FsEntry::File(sz, name));
                }
            },
        }
    }

    let sizes: usize = file_system
        .values()
        .filter_map(|entry| match entry {
            &FsEntry::Dir(sz, _) if sz <= 100000 => Some(sz),
            _ => None,
        })
        .sum();

    println!("size: {}", sizes);

    let needed_free: usize = 30000000;

    let total: usize = 70000000;

    let used: usize = match file_system.get(&PathBuf::from("/")).unwrap() {
        FsEntry::Dir(sz, _) => *sz,
        _ => panic!(),
    };

    let unused = total - used;

    let mut dirs = file_system
        .values()
        .filter_map(|entry| match entry {
            &FsEntry::Dir(sz, ref path) if sz >= (needed_free - unused) => Some((sz, path.clone())),
            _ => None,
        })
        .collect::<Vec<(usize, String)>>();

    dirs.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0));
    println!("size of smallest bigger: {}", dirs[0].0);
}
