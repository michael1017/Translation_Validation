use std::ptr;
use std::alloc::{alloc_zeroed, Layout};
use std::ffi::CString;

#[repr(C)]
struct Buffer {
    len: usize,
    alloc: *mut u8,
    data: *mut u8,
}

impl Buffer {
    #[cfg(kani)]
    #[kani::proof]
    fn new_with_size(n: usize) -> Self {
        // Calculate memory layout for the allocation
        let layout = Layout::array::<u8>(n + 1).expect("Failed to create memory layout");

        // Allocate zeroed memory and handle potential failure
        let alloc = unsafe { alloc_zeroed(layout) };
        if alloc.is_null() {
            panic!("Memory allocation failed");
        }

        Self {
            len: n,
            alloc,
            data: alloc,
        }
    }
}
