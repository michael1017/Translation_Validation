use std::cmp;
use std::ptr;

struct Node {
    key: i32,
    left: *mut Node,
    right: *mut Node,
    height: i32,
}

impl Node {
    fn new(key: i32) -> *mut Node {
        Box::into_raw(Box::new(Node {
            key,
            left: ptr::null_mut(),
            right: ptr::null_mut(),
            height: 1,
        }))
    }
}

fn max(a: i32, b: i32) -> i32 {
    cmp::max(a, b)
}

fn height(node: *mut Node) -> i32 {
    if node.is_null() {
        return 0;
    }
    unsafe { (*node).height }
}

fn right_rotate(y: *mut Node) -> *mut Node {
    unsafe {
        let x = (*y).left;
        let t2 = (*x).right;

        // Perform rotation and update heights
        (*y).left = t2;
        (*y).height = max(height((*y).left), height((*y).right)) + 1;
        (*x).right = y;
        (*x).height = max(height((*x).left), height((*x).right)) + 1;

        // Return new root
        x
    }
}

fn get_balance(node: *mut Node) -> i32 {
    if node.is_null() {
        return 0;
    }
    unsafe { height((*node).left) - height((*node).right) }
}

fn left_rotate(x: *mut Node) -> *mut Node {
    unsafe {
        let y = (*x).right;
        let t2 = (*y).left;

        // Perform rotation and update heights
        (*x).right = t2;
        (*x).height = max(height((*x).left), height((*x).right)) + 1;
        (*y).left = x;
        (*y).height = max(height((*y).left), height((*y).right)) + 1;

        // Return new root
        y
    }
}

fn insert(node: *mut Node, key: i32) -> *mut Node {
    if node.is_null() {
        return Node::new(key);
    }

    unsafe {
        if key < (*node).key {
            (*node).left = insert((*node).left, key);
        } else if key > (*node).key {
            (*node).right = insert((*node).right, key);
        } else {
            return node;
        }

        (*node).height = 1 + max(height((*node).left), height((*node).right));

        let balance = get_balance(node);

        // Left Left Case
        if balance > 1 && key < (*(*node).left).key {
            return right_rotate(node);
        }

        // Right Right Case
        if balance < -1 && key > (*(*node).right).key {
            return left_rotate(node);
        }

        // Left Right Case
        if balance > 1 && key > (*(*node).left).key {
            (*node).left = left_rotate((*node).left);
            return right_rotate(node);
        }

        // Right Left Case
        if balance < -1 && key < (*(*node).right).key {
            (*node).right = right_rotate((*node).right);
            return left_rotate(node);
        }
    }

    node
}

fn main() {
    let mut root: *mut Node = ptr::null_mut();
    root = insert(root, 7);
}