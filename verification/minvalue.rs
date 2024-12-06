use std::ptr;

#[repr(C)]
struct Node {
    key: i32,
    left: *mut Node,
    right: *mut Node,
}

impl Node {
    #[no_mangle]
    fn min_value_node_rust(node: *mut Node) -> *mut Node {
        let mut current = node;

        // loop down to find the leftmost leaf
        while !current.is_null() && unsafe { (*current).left } != ptr::null_mut() {
            current = unsafe { (*current).left };
        }

        current
    }
}

#[cfg(kani)]
#[kani::proof]
fn main() {
    // Call the min_value_node_rust function
    let mut root: *mut Node = std::ptr::null_mut();
    let _result = Node::min_value_node_rust(root);
}