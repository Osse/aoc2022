// use std::cell::RefCell;
use std::ops::FnMut;
// use std::rc::Rc;

/// A tree that contains a hierarchy of elements of type `T`.
///
/// The root of the tree is a parentless [RawNode<T>] with a list of child nodes.
/// All other nodes has a parent, and a list of child nodes.
#[derive(Debug)]
pub struct Tree<T> {
    root: RawNode<T>,
}

impl<T> Tree<T> {
    /// Creates a new tree with a root node containing the given data.
    pub fn new(data: T) -> Self {
        Tree {
            root: RawNode {
                data,
                // parent_: RefCell::new(None),
                children: Default::default(),
            },
        }
    }

    /// Returns an [Node<T>] to the root node of the tree.
    pub fn root(&self) -> Node<T> {
        Node {
            inner: &self.root,
            parent: None,
        }
    }

    /// Returns an [NodeMut<T>] to the root node of the tree.
    pub fn root_mut(&mut self) -> NodeMut<T> {
        NodeMut {
            inner: &mut self.root,
            parent: None,
        }
    }

    /// Appends a new child to this node, and return an Rc to it.
    pub fn append_child(&mut self, node: NodeMut<T>, data: T) {
        node.inner.append_child(data)
    }

    /// Inserts a new child to this node at the given index, and return a [Node<T>] to it.
    pub fn insert_child(&mut self, node: NodeMut<T>, index: usize, data: T) {
        node.inner.insert_child(index, data);
    }

    /// Removes the child node by the given index and returns it.
    /// Can be inserted elsewhere into the tree later.
    fn remove(&mut self, node: NodeMut<T>, index: usize) -> RawNode<T> {
        node.inner.children.remove(index)
    }

    /// Walk the tree depth-first, visiting each node of the tree. The function
    /// gets a shared reference to the data of each node and the level.
    pub fn walk<F>(&self, mut f: F)
    where
        F: FnMut(usize, &T),
    {
        self.root.walk_inner(0, &mut f)
    }

    // Move the given node so that its new parent is `new_parent`.
    // pub fn move_node(&self, n: &Rc<Node<T>>, new_parent: &Rc<Node<T>>) {
    //     let p = n.parent().unwrap();
    //     let index = n.index();

    //     self.move_node_by_index(&p, index, new_parent);
    // }

    // fn move_node_by_index(&self, old_parent: &Rc<Node<T>>, index: usize, new_parent: &Rc<Node<T>>) {
    //     let n = old_parent.remove(index);
    //     new_parent.append_node(n);
    // }
}

/// A node in the [Tree<T>].
struct RawNode<T> {
    data: T,
    children: Vec<RawNode<T>>,
}

impl<T> std::fmt::Debug for RawNode<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("data", &self.data)
            .field("children", &self.children)
            .finish()
    }
}

impl<T> RawNode<T> {
    /// Create a new node with the given data and parent.
    fn new(data: T) -> Self {
        RawNode {
            data,
            children: Default::default(),
        }
    }

    /// Appends a new child to this node, and return an Rc to it.
    fn append_child(&mut self, data: T) {
        let n = RawNode::new(data);
        self.children.push(n);
    }

    /// Inserts a new child to this node at the given index, and return a [Node<T>] to it.
    fn insert_child(&mut self, index: usize, data: T) -> Node<T> {
        self.children.insert(index, RawNode::new(data));

        Node {
            inner: &self.children[index],
            parent: Some(self),
        }
    }

    /// Appends a node to this node. The Rc to the node will be consumed.
    // fn append_node(self: &Rc<Node<T>>, node: Rc<Node<T>>) {
    //     self.children.push(node.clone());
    // }

    /// Inserts a node to this node. The Rc to the node will be consumed.
    // fn insert_node(self: &Rc<Node<T>>, index: usize, node: Rc<Node<T>>) {
    //     self.children.insert(index, node.clone());
    // }

    /// Returns the number of children of this node.
    fn child_len(&self) -> usize {
        self.children.len()
    }

    /// Walk the tree depth-first, starting at this node, visiting each node.
    /// The function gets a shared reference to the data of each node and the
    /// level. Note that the level of this node is zero even if it's not
    /// the root of the tree.
    pub fn walk<F>(&self, mut f: F)
    where
        F: FnMut(usize, &T),
    {
        self.walk_inner(0, &mut f);
    }

    fn walk_inner<F>(&self, level: usize, f: &mut F)
    where
        F: FnMut(usize, &T),
    {
        f(level, &self.data);
        for c in self.children.iter() {
            c.walk_inner(level + 1, f);
        }
    }

    // fn index(self: &Rc<Node<T>>) -> usize {
    //     let p = &self.parent_;
    //     let p = p.as_ref().unwrap().clone();

    //     let ret = p
    //         .children
    //         .borrow()
    //         .iter()
    //         .position(|n| Rc::ptr_eq(self, n))
    //         .unwrap();
    //     ret
    // }
}

/// A node in the [Tree<T>].
pub struct Node<'a, T> {
    inner: &'a RawNode<T>,
    parent: Option<&'a RawNode<T>>,
}

/// A mutable node in the [Tree<T>].
pub struct NodeMut<'a, T> {
    inner: &'a mut RawNode<T>,
    parent: Option<&'a RawNode<T>>,
}

impl<'a, T> Node<'a, T> {
    /// Get a node by the given index. Panics if the index is invalid.
    pub fn child(&self, index: usize) -> Node<'a, T> {
        let inner_child = self.inner.children.get(index).expect("valid index");
        Node {
            inner: inner_child,
            parent: Some(self.inner),
        }
    }

    /// Get a reference to the inner data
    pub fn data(&self) -> &'a T {
        &self.inner.data
    }

    pub fn parent(&self) -> Option<Node<T>> {
        let n = Node {
            inner: self.parent.unwrap(),
            parent: None,
        };

        Some(n)
    }

    pub fn child_len(&self) -> usize {
        self.inner.child_len()
    }
}

impl<T> NodeMut<'_, T> {
    /// Get a mutable reference to the inner data
    pub fn data_mut(&mut self) -> &mut T {
        &mut self.inner.data
    }

    pub fn child_mut(&mut self, index: usize) -> NodeMut<T> {
        let inner_child = self.inner.children.get_mut(index).expect("valid index");
        NodeMut {
            inner: inner_child,
            parent: Some(self.inner),
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
    fn insert_children() {
        let mut t: Tree<String> = Tree::new(String::from("hello"));
        let mut r = t.root();
        for s in &["child1", "child2", "child3"] {
            r.append_child(s.to_string());
        }

        assert_eq!(r.child_len(), 3);

        r.insert_child(1, "child4".to_string());

        assert_eq!(r.child(1).data, String::from("child4"));
    }

    // #[test]
    // fn index() {
    //     let mut t: Tree<String> = Tree::new(String::from("hello"));
    //     let r = t.root();
    //     for s in &["child1", "child2", "child3"] {
    //         r.append_child(s.to_string());
    //     }

    //     let n1 = r.child(1);
    //     let n2 = r.child(2);

    //     assert_eq!(n1.index(), 1);
    //     assert_eq!(n2.index(), 2);
    // }

    // #[test]
    // fn parents() {
    //     let mut t: Tree<String> = Tree::new(String::from("hello"));
    //     let r = t.root();
    //     for s in &["child1", "child2", "child3"] {
    //         r.append_child(s.to_string());
    //     }

    //     let p = r.parent();

    //     assert!(p.is_none());

    //     let c = r.child(1);
    //     let p1 = c.parent();

    //     assert!(p1.is_some());

    //     let c = r.child(2);
    //     let p2 = c.parent();

    //     assert!(p2.is_some());

    //     assert!(Rc::ptr_eq(&p1.unwrap(), &p2.unwrap()));
    // }

    // #[test]
    // fn remove_children() {
    //     let mut t: Tree<String> = Tree::new(String::from("hello"));
    //     let r = t.root_mut();
    //     for s in &["child1", "child2", "child3"] {
    //         t.append_child(r, s.to_string());
    //     }

    //     let c1 = r.child_mut(1);
    //     t.append_child(c1, String::from("rofl"));
    //     t.append_child(c1, String::from("mao"));

    //     let removed = t.remove(r, 0);

    //     assert_eq!(r.child_len(), 2);
    //     // assert!(Rc::ptr_eq(&c1, &r.child(0)), "first child is now zeroth");

    //     let _ = removed;
    // }

    // #[test]
    // fn move_child() {
    //     let mut t: Tree<String> = Tree::new(String::from("hello"));
    //     let r = t.root_mut();
    //     for s in &["child1", "child2", "child3"] {
    //         r.append_child(s.to_string());
    //     }

    //     let c1 = r.child_mut(1);

    //     t.append_child(c1, String::from("rofl"));
    //     t.append_child(c1, String::from("mao"));

    //     let removed = t.remove(r, 2);

    // r.child(1).append_node(removed);

    // let c_to_move = c1.child(2);

    // assert!(
    //     Rc::ptr_eq(&c_to_move.parent().unwrap(), &c1),
    //     "old parent is c1"
    // );

    // assert_eq!(c_to_move.data, String::from("child3"));

    // t.move_node(&c_to_move, &r);

    // assert_eq!(r.child_len(), 3);
    // assert!(
    //     Rc::ptr_eq(&c_to_move.parent().unwrap(), &r),
    //     "new parent is root"
    // );
    // }

    #[test]
    fn walk_tree() {
        let mut t: Tree<String> = Tree::new(String::from("hello"));
        let r = t.root_mut();
        for s in &["child1", "child2", "child3"] {
            t.append_child(r, s.to_string());
        }

        let c1 = r.child_mut(1);
        t.append_child(c1, String::from("rofl"));
        t.append_child(c1, String::from("mao"));

        let c2 = r.child_mut(2);
        t.append_child(c2, String::from("rofl2"));
        t.append_child(c2, String::from("mao2"));

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
  rofl2
  mao2
"#,
        );

        assert_eq!(fasit, result);
    }

    //     #[test]
    //     fn walk_node() {
    //         let mut t: Tree<String> = Tree::new(String::from("hello"));
    //         let r = t.root();
    //         for s in &["child1", "child2", "child3"] {
    //             r.append_child(s.to_string());
    //         }

    //         let c1 = r.child(1);
    //         c1.append_child(String::from("rofl"));
    //         c1.append_child(String::from("mao"));

    //         let c2 = r.child(2);
    //         c2.append_child(String::from("rofl2"));
    //         c2.append_child(String::from("mao2"));

    //         let mut result = String::new();
    //         c1.walk(|level, s| {
    //             result.push_str(&" ".repeat(level));
    //             result.push_str(s);
    //             result.push('\n');
    //         });

    //         let fasit = String::from(
    //             r#"child2
    //  rofl
    //  mao
    // "#,
    //         );

    //         assert_eq!(fasit, result);
    //     }
}
