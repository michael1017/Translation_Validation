use std::ptr;
use std::alloc::{self, Layout};
use std::ffi::CString;

struct HtEntry {
    key: *const i8,  // key is NULL if this slot is empty
    value: *mut u8,
}

struct Ht {
    entries: *mut HtEntry,  // hash slots
    capacity: usize,        // size of _entries array
    length: usize,          // number of items in hash table
}

const INITIAL_CAPACITY: usize = 16;  // must not be zero

impl Ht {
    fn create() -> Option<*mut Ht> {
        // Allocate space for hash table struct.
        let layout = Layout::new::<Ht>();
        let table = unsafe { alloc::alloc(layout) as *mut Ht };
        if table.is_null() {
            return None;
        }
        unsafe {
            (*table).length = 0;
            (*table).capacity = INITIAL_CAPACITY;

            // Allocate (zero'd) space for entry buckets.
            let entries_layout = Layout::array::<HtEntry>(INITIAL_CAPACITY).unwrap();
            let entries = alloc::alloc_zeroed(entries_layout) as *mut HtEntry;
            if entries.is_null() {
                alloc::dealloc(table as *mut u8, layout); // error, free table before we return!
                return None;
            }

            (*table).entries = entries;
        }
        Some(table)
    }

    fn destroy(table: *mut Ht) {
        unsafe {
            // First free allocated keys.
            for i in 0..(*table).capacity {
                let entry = &mut *(*table).entries.add(i);
                if !entry.key.is_null() {
                    CString::from_raw(entry.key as *mut i8); // free the CString
                }
            }

            // Then free entries array and table itself.
            let entries_layout = Layout::array::<HtEntry>((*table).capacity).unwrap();
            alloc::dealloc((*table).entries as *mut u8, entries_layout);

            let ht_layout = Layout::new::<Ht>();
            alloc::dealloc(table as *mut u8, ht_layout);
        }
    }
}

fn main() {}