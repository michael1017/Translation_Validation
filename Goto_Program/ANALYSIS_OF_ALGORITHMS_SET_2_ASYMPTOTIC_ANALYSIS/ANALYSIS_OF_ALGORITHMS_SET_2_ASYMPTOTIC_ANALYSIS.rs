use std::cmp::{min, max};
use std::slice;

fn cmpfunc(a: &i32, b: &i32) -> std::cmp::Ordering {
    a.cmp(b)
}

fn sort(arr: &mut [i32]) {
    arr.sort_by(cmpfunc);
}

fn f_gold(arr: &[i32; 4], n: usize, x: i32) -> i32 {
    for i in 0..n {
        if arr[i] == x {
            return i as i32;
        }
    }
    -1
}

fn f_filled(arr: &[i32], n: usize, x: i32) -> i32 {
    // The body of this function should be filled with the actual logic
    unimplemented!()
}

#[cfg(kani)]
#[kani::proof]
fn foo() -> i32{
    let param0_0 = [4,5,5,11];
    f_gold(&param0_0, 1, 2)
}