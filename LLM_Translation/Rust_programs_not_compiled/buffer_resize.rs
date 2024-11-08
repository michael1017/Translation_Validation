use std::ptr;
use std::ffi::CString;

struct Buffer {
    len: usize,
    alloc: *mut u8,
    data: *mut u8,
}

const BUFFER_DEFAULT_SIZE: usize = 64;

fn nearest_multiple_of(a: usize, b: usize) -> usize {
    (b + (a - 1)) & !(a - 1)
}

impl Buffer {
    fn resize(&mut self, n: usize) -> i32 {
        let new_size = nearest_multiple_of(1024, n);
        self.len = new_size;
        let new_alloc = unsafe { libc::realloc(self.alloc as *mut libc::c_void, new_size + 1) as *mut u8 };
        
        if new_alloc.is_null() {
            return -1;
        }
        
        self.alloc = new_alloc;
        self.data = new_alloc;
        
        unsafe {
            *self.alloc.add(new_size) = 0;
        }
        
        0
    }

    fn append_n(&mut self, str: &str, len: usize) -> i32 {
        let prev_len = unsafe { libc::strlen(self.data as *const libc::c_char) as usize };
        let needed = len + prev_len;

        if self.len > needed {
            unsafe {
                libc::strncat(self.data as *mut libc::c_char, CString::new(str).unwrap().as_ptr(), len);
            }
            return 0;
        }

        let ret = self.resize(needed);
        if ret == -1 {
            return -1;
        }
        
        unsafe {
            libc::strncat(self.data as *mut libc::c_char, CString::new(str).unwrap().as_ptr(), len);
        }
        
        0
    }

    fn prepend(&mut self, str: &str) -> i32 {
        let len = str.len();
        let prev_len = unsafe { libc::strlen(self.data as *const libc::c_char) as usize };
        let needed = len + prev_len;

        if self.len <= needed {
            let ret = self.resize(needed);
            if ret == -1 {
                return -1;
            }
        }

        unsafe {
            libc::memmove(self.data.add(len) as *mut libc::c_void, self.data as *const libc::c_void, prev_len + 1);
            libc::memcpy(self.data as *mut libc::c_void, CString::new(str).unwrap().as_ptr() as *const libc::c_void, len);
        }
        
        0
    }
}

fn main() {}