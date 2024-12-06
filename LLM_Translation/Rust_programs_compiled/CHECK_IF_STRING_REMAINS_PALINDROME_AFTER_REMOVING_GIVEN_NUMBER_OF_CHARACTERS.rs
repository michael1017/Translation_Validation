// Copyright (c) 2019-present, Facebook, Inc.
// All rights reserved.
//
// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.
//

use std::cmp::{min, max};
use std::ffi::CStr;
use std::os::raw::c_char;

fn cmpfunc(a: &i32, b: &i32) -> std::cmp::Ordering {
    a.cmp(b)
}

fn len<T>(arr: &[T]) -> usize {
    arr.len()
}

fn sort(arr: &mut [i32]) {
    arr.sort_by(cmpfunc);
}

fn f_gold(str: *const c_char, n: usize) -> bool {
    unsafe {
        let c_str = CStr::from_ptr(str);
        let r_str = c_str.to_str().unwrap();
        r_str.len() >= n
    }
}

fn f_filled(_str: *const c_char, _n: usize) -> bool {
    // Implement your logic here
    false
}

fn main() {
    let mut n_success = 0;
    let parameters0 = [
        "ZCoQhuM\0",
        "7437725\0",
        "11\0",
        "buGlvR\0",
        "9\0",
        "101101010110\0",
        "YguiM\0",
        "8198\0",
        "11101\0",
        "hUInqJXNdbfP\0",
    ];
    let parameters1 = [2, 53, 30, 1, 92, 3, 18, 90, 71, 4];

    for i in 0..parameters0.len() {
        let c_str0 = parameters0[i].as_ptr() as *const c_char;
        if f_filled(c_str0, parameters1[i]) == f_gold(c_str0, parameters1[i]) {
            n_success += 1;
        }
        break;
    }

    println!("#Results: {} , {}", n_success, parameters0.len());
}