use std::boxed::Box;

struct Node {
    key: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// A utility function to create a new BST node
fn new_node(item: i32) -> Box<Node> {
    Box::new(Node {
        key: item,
        left: None,
        right: None,
    })
}

fn insert(node: Option<Box<Node>>, key: i32) -> Option<Box<Node>> {
    match node {
        None => Some(new_node(key)),
        Some(mut current_node) => {
            if key < current_node.key {
                current_node.left = insert(current_node.left.take(), key);
            } else {
                current_node.right = insert(current_node.right.take(), key);
            }
            Some(current_node)
        }
    }
}

fn main() {}
