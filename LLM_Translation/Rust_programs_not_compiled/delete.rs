use std::cmp;

struct Node {
    key: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    height: i32,
}

fn height(node: &Option<Box<Node>>) -> i32 {
    match node {
        Some(n) => n.height,
        None => 0,
    }
}

fn get_balance(n: &Option<Box<Node>>) -> i32 {
    match n {
        Some(node) => height(&node.left) - height(&node.right),
        None => 0,
    }
}

fn left_rotate(x: Option<Box<Node>>) -> Option<Box<Node>> {
    if let Some(mut x_node) = x {
        if let Some(mut y_node) = x_node.right.take() {
            let t2 = y_node.left.take();
            x_node.right = t2;
            x_node.height = cmp::max(height(&x_node.left), height(&x_node.right)) + 1;
            y_node.left = Some(x_node);
            y_node.height = cmp::max(height(&y_node.left), height(&y_node.right)) + 1;
            return Some(y_node);
        }
    }
    None
}

fn right_rotate(y: Option<Box<Node>>) -> Option<Box<Node>> {
    if let Some(mut y_node) = y {
        if let Some(mut x_node) = y_node.left.take() {
            let t2 = x_node.right.take();
            y_node.left = t2;
            y_node.height = cmp::max(height(&y_node.left), height(&y_node.right)) + 1;
            x_node.right = Some(y_node);
            x_node.height = cmp::max(height(&x_node.left), height(&x_node.right)) + 1;
            return Some(x_node);
        }
    }
    None
}

fn min_value_node(node: &Option<Box<Node>>) -> Option<Box<Node>> {
    let mut current = node.clone();
    while let Some(ref current_node) = current {
        if current_node.left.is_none() {
            break;
        }
        current = current_node.left.clone();
    }
    current
}

fn delete_node(root: Option<Box<Node>>, key: i32) -> Option<Box<Node>> {
    let mut root = root;
    if root.is_none() {
        return root;
    }
    if let Some(root_ref) = root.as_mut() {
        if key < root_ref.key {
            root_ref.left = delete_node(root_ref.left.take(), key);
        } else if key > root_ref.key {
            root_ref.right = delete_node(root_ref.right.take(), key);
        } else {
            if root_ref.left.is_none() || root_ref.right.is_none() {
                let temp = if root_ref.left.is_some() {
                    root_ref.left.take()
                } else {
                    root_ref.right.take()
                };
                if temp.is_none() {
                    root = None;
                } else {
                    *root_ref = *temp.unwrap();
                }
            } else {
                let temp = min_value_node(&root_ref.right).unwrap();
                root_ref.key = temp.key;
                root_ref.right = delete_node(root_ref.right.take(), temp.key);
            }
        }
    }
    if root.is_some() {
        if let Some(root_ref) = root.as_mut() {
            root_ref.height = cmp::max(height(&root_ref.left), height(&root_ref.right)) + 1;
            let balance = get_balance(root.as_ref());
            if balance > 1 && get_balance(root_ref.left.as_ref()) >= 0 {
                return right_rotate(root);
            }
            if balance > 1 && get_balance(root_ref.left.as_ref()) < 0 {
                root_ref.left = left_rotate(root_ref.left.take());
                return right_rotate(root);
            }
            if balance < -1 && get_balance(root_ref.right.as_ref()) <= 0 {
                return left_rotate(root);
            }
            if balance < -1 && get_balance(root_ref.right.as_ref()) > 0 {
                root_ref.right = right_rotate(root_ref.right.take());
                return left_rotate(root);
            }
        }
    }
    root
}

fn main() {}