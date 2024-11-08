use std::ptr;
use std::alloc::{alloc_zeroed, Layout};
use std::ffi::CString;

struct Buffer {
    len: usize,
    alloc: *mut u8,
    data: *mut u8,
}

impl Buffer {
    fn new_with_size(n: usize) -> Option<Self> {
        let layout = Layout::array::<u8>(n + 1).ok()?;
        let alloc = unsafe { alloc_zeroed(layout) };

        if alloc.is_null() {
            None
        } else {
            Some(Self {
                len: n,
                alloc,
                data: alloc,
            })
        }
    }
}

fn main() {
    // Your code here
}