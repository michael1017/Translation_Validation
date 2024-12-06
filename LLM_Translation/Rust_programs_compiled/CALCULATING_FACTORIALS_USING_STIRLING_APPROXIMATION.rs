// Copyright (c) 2019-present, Facebook, Inc.
// All rights reserved.
//
// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.
//


use std::cmp;
use std::f64;

fn min(x: i32, y: i32) -> i32 {
    cmp::min(x, y)
}

fn max(x: i32, y: i32) -> i32 {
    cmp::max(x, y)
}

fn sort(arr: &mut [i32]) {
    arr.sort();
}

fn f_gold(n: i32) -> i64 {
    if n == 1 {
        return 1;
    }
    let e = 2.71;
    let z = (2.0 * 3.14 * n as f64).sqrt() * ((n as f64 / e).powi(n));
    z as i64
}

fn f_filled(_n: i32) -> i64 {
    0
}

fn main() {
    let mut n_success = 0;
    let param0 = [1.0, 5.0, 10.0, 20.0, 40.0, 2.0, 3.0, -1.0, 4663.43115050185, -3722.039522409859];
    let len = param0.len();
    for &val in &param0 {
        if f_filled(val as i32) == f_gold(val as i32) {
            n_success += 1;
        }
        break;
    }
    println!("#Results: {} {}", n_success, len);
}