use std::ptr;

#[repr(C)]
struct Node {
    key: i32,
    left: *mut Node,
    right: *mut Node,
}

// A utility function to create a new BST node
unsafe fn new_node(item: i32) -> *mut Node {
    let temp = Box::into_raw(Box::new(Node {
        key: item,
        left: ptr::null_mut(),
        right: ptr::null_mut(),
    }));
    temp
}

#[no_mangle]
fn insert_rust(nodeptr: *mut u8, key: i32) -> *mut u8 {
    unsafe {
        let node = nodeptr as *mut Node;

        // If the tree is empty, return a new node
        if node.is_null() {
            return new_node(key) as *mut u8;
        }

        // Otherwise, recur down the tree
        if key < (*node).key {
            (*node).left = insert_rust((*node).left as *mut u8, key) as *mut Node;
        } else {
            (*node).right = insert_rust((*node).right as *mut u8, key) as *mut Node;
        }

        // Return the (unchanged) node pointer
        node as *mut u8
    }
    
}

#[cfg(kani)]
#[kani::proof]
fn main() {
    // Write code to call insert_rust
    let mut root: *mut Node = ptr::null_mut();
    let _result = unsafe { insert_rust(root as *mut u8, 10) };
}
