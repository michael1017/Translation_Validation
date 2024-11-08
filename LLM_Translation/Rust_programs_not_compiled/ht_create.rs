use std::ptr;
use std::ffi::CString;

struct HtEntry {
    key: *const i8,  // key is a C-style string, or null if this slot is empty
    value: *mut std::ffi::c_void,
}

struct Ht {
    entries: *mut HtEntry,  // hash slots
    capacity: usize,        // size of entries array
    length: usize,          // number of items in hash table
}

const INITIAL_CAPACITY: usize = 16; // must not be zero

impl Ht {
    fn create() -> *mut Ht {
        // Allocate space for hash table struct
        let table = unsafe { libc::malloc(std::mem::size_of::<Ht>()) as *mut Ht };
        if table.is_null() {
            return ptr::null_mut();
        }
        unsafe {
            (*table).length = 0;
            (*table).capacity = INITIAL_CAPACITY;

            // Allocate (zero'd) space for entry buckets
            (*table).entries = libc::calloc((*table).capacity, std::mem::size_of::<HtEntry>()) as *mut HtEntry;
            if (*table).entries.is_null() {
                libc::free(table as *mut std::ffi::c_void); // error, free table before we return!
                return ptr::null_mut();
            }
        }
        table
    }

    fn destroy(table: *mut Ht) {
        unsafe {
            // First free allocated keys
            for i in 0..(*table).capacity {
                let key_ptr = (*table).entries.add(i).as_mut().unwrap().key;
                if !key_ptr.is_null() {
                    CString::from_raw(key_ptr as *mut i8);
                }
            }
        
            // Then free entries array and table itself
            libc::free((*table).entries as *mut std::ffi::c_void);
            libc::free(table as *mut std::ffi::c_void);
        }
    }
}

fn main() {}