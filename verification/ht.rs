use std::os::raw::c_char;
use std::ffi::CString;

const FNV_OFFSET: u64 = 14695981039346656037;
const FNV_PRIME: u64 = 1099511628211;

#[no_mangle]
fn hash_key_rust(key: *const c_char) -> u64 {
    let mut hash = FNV_OFFSET;
    let mut p = key;

    unsafe {
        while *p != 0 {
            hash ^= (*p as u8) as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
            p = p.add(1);
        }
    }

    hash
}

#[cfg(kani)]
#[kani::proof]
fn main() {
    // Create a Rust string
    let input = "example";

    // Convert the Rust string into a C-compatible string
    let c_string = CString::new(input).expect("CString::new failed");

    // Call the external function
    unsafe {
        let result = hash_key_rust(c_string.as_ptr());
        println!("Hash result for '{}': {}", input, result);
    }
}