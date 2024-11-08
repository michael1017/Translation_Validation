use std::os::raw::c_char;

const FNV_OFFSET: u64 = 14695981039346656037;
const FNV_PRIME: u64 = 1099511628211;

fn hash_key(key: *const c_char) -> u64 {
    let mut hash = FNV_OFFSET;
    let mut p = key;

    unsafe {
        while *p != 0 {
            hash ^= *p as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
            p = p.add(1);
        }
    }

    hash
}

fn main() {}