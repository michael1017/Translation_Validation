use std::ptr;
use std::ffi::c_void;

const CSV_TAB: u8 = 9;
const CSV_LF: u8 = 10;
const CSV_SPACE: u8 = 10;
const CSV_CR: u8 = 10;
const MEM_BLK_SIZE: usize = 128;
const CSV_COMMA: u8 = 0x2c;
const CSV_QUOTE: u8 = 0x22;

const ROW_NOT_BEGUN: i32 = 0;
const FIELD_NOT_BEGUN: i32 = 1;
const FIELD_BEGUN: i32 = 2;
const FIELD_MIGHT_HAVE_ENDED: i32 = 3;

struct CsvParser {
    pstate: i32,
    quoted: i32,
    spaces: usize,
    entry_buf: *mut u8,
    entry_pos: usize,
    entry_size: usize,
    status: i32,
    options: u8,
    quote_char: u8,
    delim_char: u8,
    is_space: Option<fn(u8) -> i32>,
    is_term: Option<fn(u8) -> i32>,
    blk_size: usize,
    malloc_func: Option<fn(usize) -> *mut c_void>,
    realloc_func: Option<fn(*mut c_void, usize) -> *mut c_void>,
    free_func: Option<fn(*mut c_void)>,
}

struct Counts {
    fields: u64,
    rows: u64,
}

fn cb1(_s: *mut c_void, _len: usize, data: *mut c_void) {
    unsafe {
        let counts = &mut *(data as *mut Counts);
        counts.fields += 1;
    }
}

fn cb2(_c: i32, data: *mut c_void) {
    unsafe {
        let counts = &mut *(data as *mut Counts);
        counts.rows += 1;
    }
}

fn csv_init(p: &mut CsvParser, options: u8) -> i32 {
    if p as *mut CsvParser == ptr::null_mut() {
        return -1;
    }

    p.entry_buf = ptr::null_mut();
    p.pstate = ROW_NOT_BEGUN;
    p.quoted = 0;
    p.spaces = 0;
    p.entry_pos = 0;
    p.entry_size = 0;
    p.status = 0;
    p.options = options;
    p.quote_char = CSV_QUOTE;
    p.delim_char = CSV_COMMA;
    p.is_space = None;
    p.is_term = None;
    p.blk_size = MEM_BLK_SIZE;
    p.malloc_func = None;
    p.realloc_func = Some(libc::realloc);
    p.free_func = Some(libc::free);

    0
}

fn main() {}