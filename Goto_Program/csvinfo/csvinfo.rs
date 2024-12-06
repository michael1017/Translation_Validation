use std::ptr;
use std::alloc::{alloc, dealloc, realloc, Layout};

const CSV_TAB: u8 = 9;
const CSV_LF: u8 = 10;
const CSV_SPACE: u8 = 10;
const CSV_CR: u8 = 10;

#[repr(C)]
struct CsvParser {
    pstate: i32,                  // Parser state
    quoted: i32,                  // Is the current field a quoted field?
    spaces: usize,                // Number of continuous spaces after quote or in a non-quoted field
    entry_buf: *mut u8,           // Entry buffer
    entry_pos: usize,             // Current position in entry_buf (and current size of entry)
    entry_size: usize,            // Size of entry buffer
    status: i32,                  // Operation status
    options: u8,
    quote_char: u8,
    delim_char: u8,
    is_space: Option<fn(u8) -> i32>,
    is_term: Option<fn(u8) -> i32>,
    blk_size: usize,
    malloc_func: Option<fn(usize) -> *mut u8>,
    realloc_func: Option<fn(*mut u8, usize) -> *mut u8>,
    free_func: Option<fn(*mut u8)>,
}

impl CsvParser {
    #[cfg(kani)]
    #[kani::proof]
    #[no_mangle]
    fn csv_get_opts(&self) -> i32 {
        // Return the currently set options of parser
        if ptr::eq(self as *const _, ptr::null()) {
            return -1;
        }
        self.options as i32
    }
}
