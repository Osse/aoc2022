mod tree;

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
    Dir(String),
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
    // let mut t = tree::Tree::<FsEntry>::new(0);

    let mut path = String::new();
    for l in contents.lines().map(parse_line) {
        match l {
            Line::Command(c) => match c {
                Command::Cd(dir) => {
                    if dir.starts_with("/") {
                        path = dir;
                    }
                }
                Command::Ls => (),
            },
            Line::Output(o) => match o {
                Output::Dir(dir) => {}
                Output::File(sz, name) => {}
            },
        }
    }

    // let r = t.root();
    // r.append_child(5);
    // r.append_child(6);

    // let c = r.child(1);

    // c.append_child(7);
    // c.append_child(8);

    // r.append_child(9);

    // t.walk(|level, number| println!("{}{}", " ".repeat(level * 4), number));
}
