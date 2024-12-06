// Copyright (c) 2019-present, Facebook, Inc.
// All rights reserved.
//
// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.
//

use std::f64;

fn min(x: i32, y: i32) -> i32 {
    if x < y { x } else { y }
}

fn max(x: i32, y: i32) -> i32 {
    if x > y { x } else { y }
}

fn cmpfunc(a: &i32, b: &i32) -> std::cmp::Ordering {
    a.cmp(b)
}

fn len(arr: &[i32]) -> usize {
    arr.len()
}

fn sort(arr: &mut [i32]) {
    arr.sort_by(cmpfunc)
}

fn f_gold(side: i32) -> f64 {
    ((15.0 + (7.0 * (5.0f64.sqrt()))) / 4.0) * ((side as f64).powi(3))
}

fn f_filled(side: i32) -> f64 {
    // Implementation goes here
    0.0
}

fn main() {
    let mut n_success = 0;
    let param0 = [56, 73, 22, 10, 84, 20, 51, 91, 10, 83];
    for &val in &param0 {
        if (1.0 - (0.0000001 + f_gold(val).abs()) / (f_filled(val).abs() + 0.0000001)).abs() < 0.001 {
            n_success += 1;
        }
        break;
    }
    println!("#Results: {} {}", n_success, len(&param0));
}