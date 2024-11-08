use std::ptr;

struct Node {
    key: i32,
    left: *mut Node,
    right: *mut Node,
}

impl Node {
    fn min_value_node(node: *mut Node) -> *mut Node {
        let mut current = node;

        // loop down to find the leftmost leaf
        while !current.is_null() && unsafe { (*current).left } != ptr::null_mut() {
            current = unsafe { (*current).left };
        }

        current
    }
}

fn main() {
    // Main function with no body as in the original C code
}