use std::cell::RefCell;
use std::ops::FnMut;
use std::rc::Rc;

#[derive(Debug)]
pub struct Tree<T> {
    root: Node<T>,
}

impl<T> Tree<T>
where
    T: std::fmt::Debug,
{
    pub fn new(data: T) -> Self {
        Tree {
            root: Node {
                data,
                parent_: RefCell::new(None),
                children: RefCell::default(),
            },
        }
    }

    pub fn root<'a>(&mut self) -> Rc<Node<T>> {
        unsafe { Rc::from_raw(&self.root) }
    }

    pub fn walk<F>(&self, mut f: F)
    where
        F: FnMut(usize, &T),
    {
        self.root.walk(0, &mut f)
    }

    pub fn move_node(&self, n: &Rc<Node<T>>, new_parent: &Rc<Node<T>>) {
        let p = n.parent().unwrap();
        let index = n.index();

        self.move_node_by_index(&p, index, new_parent);
    }

    fn move_node_by_index(&self, old_parent: &Rc<Node<T>>, index: usize, new_parent: &Rc<Node<T>>) {
        let n = old_parent.remove(index);
        new_parent.append_node(n);
    }
}

pub struct Node<T> {
    data: T,
    parent_: RefCell<Option<Rc<Node<T>>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> std::fmt::Debug for Node<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("data", &self.data)
            .field("parent", &self.parent_.borrow().is_some())
            .field("children", &self.children)
            .finish()
    }
}

impl<T> Node<T>
where
    T: std::fmt::Debug,
{
    fn new(data: T, parent: Rc<Node<T>>) -> Self {
        Node {
            data,
            parent_: RefCell::new(Some(parent)),
            children: RefCell::default(),
        }
    }

    pub fn append_child(&self, data: T) {
        let n = Node::new(data, unsafe { Rc::from_raw(&*self) });
        self.children.borrow_mut().push(Rc::new(n));
    }

    pub fn append_node(self: &Rc<Node<T>>, node: Rc<Node<T>>) {
        self.children.borrow_mut().push(node.clone());
        *node.parent_.borrow_mut() = Some(self.clone());
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

    pub fn walk<F>(&self, level: usize, f: &mut F)
    where
        F: FnMut(usize, &T),
    {
        f(level, &self.data);
        for c in self.children.borrow().iter() {
            c.walk(level + 1, f);
        }
    }

    pub fn parent(&self) -> Option<Rc<Node<T>>> {
        let kek = self.parent_.borrow();
        match kek.as_ref() {
            Some(p) => Some(p.clone()),
            None => None,
        }
    }

    fn index(self: &Rc<Node<T>>) -> usize {
        let p = &self.parent_;
        let p = p.borrow().as_ref().unwrap().clone();

        let ret = p
            .children
            .borrow()
            .iter()
            .position(|n| Rc::ptr_eq(self, n))
            .unwrap();
        ret
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
    fn index() {
        let mut t: Tree<String> = Tree::new(String::from("hello"));
        let r = t.root();
        for s in &["child1", "child2", "child3"] {
            r.append_child(s.to_string());
        }

        let n1 = r.child(1);
        let n2 = r.child(2);

        assert_eq!(n1.index(), 1);
        assert_eq!(n2.index(), 2);
    }

    #[test]
    fn parents() {
        let mut t: Tree<String> = Tree::new(String::from("hello"));
        let r = t.root();
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
        let r = t.root();
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
        let r = t.root();
        for s in &["child1", "child2", "child3"] {
            r.append_child(s.to_string());
        }

        let c1 = r.child(1);

        c1.append_child(String::from("rofl"));
        c1.append_child(String::from("mao"));

        let removed = r.remove(2);

        r.child(1).append_node(removed);

        let c_to_move = c1.child(2);

        assert!(
            Rc::ptr_eq(&c_to_move.parent().unwrap(), &c1),
            "old parent is c1"
        );

        assert_eq!(c_to_move.data, String::from("child3"));

        t.move_node(&c_to_move, &r);

        assert_eq!(r.child_len(), 3);
        assert!(Rc::ptr_eq(&kek.parent().unwrap(), &r), "new parent is root");
    }

    #[test]
    fn walk() {
        let mut t: Tree<String> = Tree::new(String::from("hello"));
        let r = t.root();
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
