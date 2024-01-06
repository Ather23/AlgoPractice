use std::env::current_exe;

#[derive(Debug, Clone)]
struct Node<T> where T: PartialEq {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    value: Option<T>,
}

enum NodeChild {
    Left,
    Right,
}

impl<T> Node<T> where T: PartialEq {
    fn new(value: T) -> Self {
        Node {
            left: None,
            right: None,
            value: Some(value),
        }
    }

    fn unwrap_value_safely(&self) -> T {
        self.value.unwrap()
    }

    fn unwrap_child_safely(&self, node_type: NodeChild) -> &Box<Node<T>> {
        match node_type {
            Left => &self.left.unwrap(),
            Right => &self.right.unwrap(),
        }
    }
}

struct Tree<T> where T: PartialEq {
    root: Option<Node<T>>,
}

impl<T> Tree<T> where T: PartialEq {
    fn new() -> Self {
        Tree { root: None }
    }

    fn insert(&mut self, value: T) {
        match &self.root {
            None => {
                self.root = Some(Node::new(value));
            }
            Some(root) => {}
        }
    }

    fn insert_traverse(&self, value: T, node: Option<Node<T>>) -> Option<Node<T>> {
        match &node {
            None => { node }
            Some(_node) => {
                if value < _node.left.unwrap().value {
                    _node.left = &self.insert_traverse(value, _node.left);
                }

                if value > _node.right.unwrap().value {
                    _node.right = &self.insert_traverse(value, _node.right);
                }
            }
        }
    }
}

fn main() {}

#[cfg(test)] // This attribute indicates that this module contains tests
mod bst_tests {
    use super::*; // Import items from the parent module

    #[test]
    fn test_insert_root_node() {
        let tree: Tree<i32> = Tree::new();

        assert!(true)
    }
}
