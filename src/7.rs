use std::cell::RefCell;
use std::ops::Fn;
use std::rc::Rc;

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

#[derive(Debug)]
struct Tree<T> {
    root: Node<T>,
}

impl<T> Tree<T> {
    fn new(data: T) -> Self {
        Tree {
            root: Node {
                data,
                parent: None,
                children: RefCell::default(),
            },
        }
    }

    fn root<'a>(&mut self) -> Rc<Node<T>> {
        unsafe { Rc::from_raw(&self.root) }
    }

    fn walk<F>(&self, f: F)
    where
        F: Fn(usize, &T),
    {
        self.root.walk(0, &f)
    }
}

struct Node<T> {
    data: T,
    parent: Option<Rc<Node<T>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> std::fmt::Debug for Node<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("data", &self.data)
            .field("parent", &self.parent.is_some())
            .field("children", &self.children)
            .finish()
    }
}

impl<T> Node<T> {
    fn new(data: T, parent: Rc<Node<T>>) -> Self {
        Node {
            data,
            parent: Some(parent),
            children: RefCell::default(),
        }
    }
    fn append_child(&self, data: T) {
        let n = Node {
            data,
            parent: Some(unsafe { Rc::from_raw(&*self) }),
            children: RefCell::default(),
        };

        self.children.borrow_mut().push(Rc::new(n));
    }

    fn child(&self, index: usize) -> Rc<Node<T>> {
        self.children
            .borrow()
            .get(index)
            .expect("valid index")
            .clone()
    }

    fn walk<F>(&self, level: usize, f: &F)
    where
        F: Fn(usize, &T),
    {
        f(level, &self.data);
        for c in self.children.borrow().iter() {
            c.walk(level + 1, f);
        }
    }
}

fn main() {
    // let contents = std::fs::read_to_string("inputs/7.txt").expect("read input");
    // for l in contents.lines().map(parse_line) {}

    let mut t = Tree::<i32>::new(0);

    let mut r = t.root();
    r.append_child(5);
    r.append_child(6);

    let mut c = r.child(1);

    c.append_child(7);
    c.append_child(8);

    r.append_child(9);

    t.walk(|level, number| println!("{}{}", " ".repeat(level * 4), number));
}
