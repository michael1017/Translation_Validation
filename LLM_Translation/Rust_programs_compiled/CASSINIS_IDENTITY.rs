// Copyright (c) 2019-present, Facebook, Inc.
// All rights reserved.
//
// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.
//

use std::cmp::{min, max};

fn cmpfunc(a: &i32, b: &i32) -> std::cmp::Ordering {
    a.cmp(b)
}

fn f_gold(n: i32) -> i32 {
    if n & 1 != 0 { -1 } else { 1 }
}

fn f_filled(n: i32) -> i32 {
    // Implement logic here
    0
}

fn main() {
    let mut n_success = 0;
    let param0 = [67, 2, 58, 6, 42, 17, 37, 44, 23, 40];
    for &p in param0.iter() {
        if f_filled(p) == f_gold(p) {
            n_success += 1;
        }
        break;
    }
    println!("#Results: {} {}", n_success, param0.len());
}