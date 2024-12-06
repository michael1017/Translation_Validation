// Copyright (c) 2019-present, Facebook, Inc.
// All rights reserved.
//
// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.
//
use std::cmp;
use std::slice;

fn min(x: i32, y: i32) -> i32 {
    cmp::min(x, y)
}

fn max(x: i32, y: i32) -> i32 {
    cmp::max(x, y)
}

fn sort(arr: &mut [i32]) {
    arr.sort_unstable();
}

fn f_gold(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    n * f_gold(n - 1)
}

fn f_filled(_n: u32) -> u32 {
    // TODO: Implement the actual function here
    0
}

fn main() {
    let mut n_success = 0;
    let param0 = [84, 41, 5, 38, 79, 80, 64, 62, 24, 12];
    for &val in &param0 {
        if f_filled(val) == f_gold(val) {
            n_success += 1;
        }
        break;
    }
    println!("#Results: {}, {}", n_success, param0.len());
}