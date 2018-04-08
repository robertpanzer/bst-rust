use std::cmp::Ordering::*;
use std::mem;

#[derive(Debug)]
pub struct Tree<T> {
    root: Link<T>
}

#[derive(Debug)]
enum Link<T> {
    Empty,
    NonEmpty(Box<Node<T>>),
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Ord> Tree<T> {
    pub fn new() -> Self {
        Tree { root: Link::Empty }
    }

    pub fn insert(&mut self, value: T) {
        self.root.insert(value)
    }

    pub fn delete(&mut self, value: &T) -> Option<T> {
        self.root.delete(value)
    }

    pub fn size(&self) -> usize {
        self.root.size()
    }

    pub fn contains(&self, value: &T) -> bool {
        self.root.contains(value)
    }

    pub fn iter(&self) -> TreeIterator<T> {
        let mut iter = TreeIterator { unvisited: Vec::new() };
        iter.push_left_nodes(&self.root);
        iter
    }
}

pub struct TreeIterator<'a, T: 'a> {
    unvisited: Vec<&'a Node<T>>
}

impl<'a, T: 'a> TreeIterator<'a, T> {
    fn push_left_nodes(&mut self, mut l: &'a Link<T>) {
        while let Link::NonEmpty(ref node) = *l {
            self.unvisited.push(node);
            l = &node.left;
        }
    }
}

impl<'a, T: 'a> Iterator for TreeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.unvisited.pop()
            .map(|node| {
                self.push_left_nodes(&node.right);
                &node.value
            })
    }
}

impl<'a, T: 'a + Ord> IntoIterator for &'a Tree<T> {
    type Item = &'a T;
    type IntoIter = TreeIterator<'a, T>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        self.iter()
    }
}

impl<T: Ord> Link<T> {
    fn insert(&mut self, value: T) {
        match *self {
            Link::Empty =>
                *self = Link::NonEmpty(Box::new(Node { value, left: Link::Empty, right: Link::Empty })),
            Link::NonEmpty(ref mut boxed_node) =>
                match value.cmp(&boxed_node.value) {
                    Equal => (),
                    Greater => boxed_node.right.insert(value),
                    Less => boxed_node.left.insert(value),
                }
        }
    }

    fn delete(&mut self, value: &T) -> Option<T> {
        match mem::replace(self, Link::Empty) {
            Link::Empty => None,
            Link::NonEmpty(mut boxed_node) => {
                match value.cmp(&boxed_node.value) {
                    Equal => {
                        // self points to the node to delete
                        if boxed_node.is_leaf() {
                            // Easy, node is a leaf, just return the value, self is already replaced with Empty
                            Some(boxed_node.value)
                        } else if boxed_node.has_left_child() && !boxed_node.has_right_child() {
                            // Node has no right child, just let this Link point to the left child
                            *self = mem::replace(&mut boxed_node.left, Link::Empty);
                            Some(boxed_node.value)
                        } else if !boxed_node.has_left_child() && boxed_node.has_right_child() {
                            // Node has no left child, just let this Link point to the right child
                            *self = mem::replace(&mut boxed_node.right, Link::Empty);
                            Some(boxed_node.value)
                        } else {
                            // The most complex case, delete the minimum node in the right subtree and
                            // replace its value in this node
                            let min_node = boxed_node.right.delete_min().unwrap();
                            let old_value = mem::replace(&mut boxed_node.value, min_node.value);
                            *self = Link::NonEmpty(boxed_node);
                            Some(old_value)
                        }
                    }
                    Less => {
                        // The value to delete is in the left subtree -> recurse
                        let result = boxed_node.left.delete(value);
                        *self = Link::NonEmpty(boxed_node);
                        result
                    }
                    Greater => {
                        // The value to delete is in the right subtree -> recurse
                        let result = boxed_node.right.delete(value);
                        *self = Link::NonEmpty(boxed_node);
                        result
                    }
                }
            }
        }
    }

    //
    // A helper function that is called by delete() to delete the minimum node in the right subtree
    // when the the node to delete has two children.
    fn delete_min(&mut self) -> Option<Box<Node<T>>> {
        match mem::replace(self, Link::Empty) {
            Link::Empty => None,
            Link::NonEmpty(mut boxed_node) => {
                if boxed_node.is_leaf() {
                    Some(boxed_node)
                } else if boxed_node.has_left_child() {
                    let result = boxed_node.left.delete_min();
                    *self = Link::NonEmpty(boxed_node);
                    result
                } else {
                    // Must have a right child
                    *self = mem::replace(&mut boxed_node.right, Link::Empty);
                    Some(boxed_node)
                }
            }
        }
    }

    fn size(&self) -> usize {
        match *self {
            Link::Empty => 0,
            Link::NonEmpty(ref node) => 1 + node.left.size() + node.right.size()
        }
    }

    fn empty_tree(&self) -> bool {
        match *self {
            Link::Empty => true,
            _ => false
        }
    }

    fn contains(&self, value: &T) -> bool {
        match *self {
            Link::Empty => false,
            Link::NonEmpty(ref node) =>
                match value.cmp(&node.value) {
                    Equal => true,
                    Less => node.left.contains(value),
                    Greater => node.right.contains(value)
                }
        }
    }
}

impl<T: Ord> Node<T> {
    fn is_leaf(&self) -> bool {
        self.left.empty_tree() && self.right.empty_tree()
    }

    fn has_left_child(&self) -> bool {
        !self.left.empty_tree()
    }

    fn has_right_child(&self) -> bool {
        !self.right.empty_tree()
    }
}