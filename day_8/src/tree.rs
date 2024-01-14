use std::{cell::RefCell, fmt::Debug, rc::Rc};
use std::{fmt, fmt::Formatter};

#[derive(Clone)]
pub struct Node<T> {
    pub val: T,
    pub left: Option<NodeRef<T>>,
    pub right: Option<NodeRef<T>>,
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        Node {
            val,
            left: None,
            right: None,
        }
    }

    pub fn set_left(&mut self, left: NodeRef<T>) {
        self.left = Some(left);
    }

    pub fn set_right(&mut self, right: NodeRef<T>) {
        self.right = Some(right);
    }
}

pub type NodeRef<T> = Rc<RefCell<Node<T>>>;

impl Debug for Node<String> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let left = match &self.left {
            Some(node) => node.borrow().val.clone(),
            None => String::from("_"),
        };
        let right = match &self.right {
            Some(node) => node.borrow().val.clone(),
            None => String::from("_"),
        };
        write!(f, "{} -> ({}, {})", self.val, left, right)
    }
}
