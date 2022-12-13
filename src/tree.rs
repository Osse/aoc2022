use std::cell::RefCell;
use std::ops::FnMut;
use std::rc::Rc;

#[derive(Debug)]
pub struct Tree<T> {
    root: Node<T>,
}

impl<T> Tree<T> {
    pub fn new(data: T) -> Self {
        Tree {
            root: Node {
                data,
                parent_: None,
                children: RefCell::default(),
            },
        }
    }

    pub fn root<'a>(&mut self) -> Rc<Node<T>> {
        unsafe { Rc::from_raw(&self.root) }
    }

    pub fn walk<F>(&self, f: F)
    where
        F: FnMut(usize, &T),
    {
        self.root.walk(0, &mut f)
    }
}

pub struct Node<T> {
    data: T,
    parent_: Option<Rc<Node<T>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> std::fmt::Debug for Node<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("data", &self.data)
            .field("parent", &self.parent_.is_some())
            .field("children", &self.children)
            .finish()
    }
}

impl<T> Node<T> {
    fn new(data: T, parent: Rc<Node<T>>) -> Self {
        Node {
            data,
            parent_: Some(parent),
            children: RefCell::default(),
        }
    }

    pub fn append_child(&self, data: T) {
        let n = Node::new(data, unsafe { Rc::from_raw(&*self) });
        self.children.borrow_mut().push(Rc::new(n));
    }

    pub fn append_node(&self, node: Rc<Node<T>>) {
        self.children.borrow_mut().push(node)
    }

    pub fn remove(&self, index: usize) -> Rc<Node<T>> {
        self.children.borrow_mut().remove(index)
    }

    pub fn child(&self, index: usize) -> Rc<Node<T>> {
        self.children
            .borrow()
            .get(index)
            .expect("valid index")
            .clone()
    }

    pub fn child_len(&self) -> usize {
        self.children.borrow().len()
    }

    fn walk<F>(&self, level: usize, f: &mut F)
    where
        F: FnMut(usize, &T),
    {
        f(level, &self.data);
        for c in self.children.borrow().iter() {
            c.walk(level + 1, f);
        }
    }

    fn parent(&self) -> Option<Rc<Node<T>>> {
        match &self.parent_ {
            Some(p) => Some(p.clone()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
        let t: Tree<String> = Tree::new(String::from("hello"));
    }

    #[test]
    fn append_children() {
        let mut t: Tree<String> = Tree::new(String::from("hello"));
        let mut r = t.root();
        for s in &["child1", "child2", "child3"] {
            r.append_child(s.to_string());
        }

        assert_eq!(r.child_len(), 3);

        r.append_child("child4".to_string());

        assert_eq!(r.child_len(), 4);
    }

    #[test]
    fn parents() {
        let mut t: Tree<String> = Tree::new(String::from("hello"));
        let mut r = t.root();
        for s in &["child1", "child2", "child3"] {
            r.append_child(s.to_string());
        }

        let p = r.parent();

        assert!(p.is_none());

        let c = r.child(1);
        let p1 = c.parent();

        assert!(p1.is_some());

        let c = r.child(2);
        let p2 = c.parent();

        assert!(p2.is_some());

        assert!(Rc::ptr_eq(&p1.unwrap(), &p2.unwrap()));
    }

    #[test]
    fn remove_children() {
        let mut t: Tree<String> = Tree::new(String::from("hello"));
        let mut r = t.root();
        for s in &["child1", "child2", "child3"] {
            r.append_child(s.to_string());
        }

        let c1 = r.child(1);
        c1.append_child(String::from("rofl"));
        c1.append_child(String::from("mao"));

        let removed = r.remove(0);

        assert_eq!(r.child_len(), 2);
        assert!(Rc::ptr_eq(&c1, &r.child(0)), "first child is now zeroth");

        let _ = removed;
    }

    #[test]
    fn move_child() {
        let mut t: Tree<String> = Tree::new(String::from("hello"));
        let mut r = t.root();
        for s in &["child1", "child2", "child3"] {
            r.append_child(s.to_string());
        }

        let c1 = r.child(1);
        c1.append_child(String::from("rofl"));
        c1.append_child(String::from("mao"));

        let removed = r.remove(2);

        r.child(1).append_node(removed);

        assert_eq!(c1.child(2).data, String::from("child3"));
    }

    #[test]
    fn walk() {
        let mut t: Tree<String> = Tree::new(String::from("hello"));
        let mut r = t.root();
        for s in &["child1", "child2", "child3"] {
            r.append_child(s.to_string());
        }

        let c1 = r.child(1);
        c1.append_child(String::from("rofl"));
        c1.append_child(String::from("mao"));

        let mut result = String::new();
        t.walk(|level, s| {
            result.push_str(&" ".repeat(level));
            result.push_str(s);
            result.push('\n');
        });

        let fasit = String::from(
            r#"hello
 child1
 child2
  rofl
  mao
 child3
"#,
        );

        assert_eq!(fasit, result);
    }
}
