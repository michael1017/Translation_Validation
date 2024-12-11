// Copyright (c) 2019-present, Facebook, Inc.
// All rights reserved.
//
// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.
//

use std::f64;
use std::cmp::{min, max};

fn cmpfunc(a: &i32, b: &i32) -> std::cmp::Ordering {
    a.cmp(b)
}

fn len(arr: &[i32]) -> usize {
    arr.len()
}

fn sort(arr: &mut [i32]) {
    arr.sort_by(cmpfunc);
}

#[no_mangle]
fn f_gold(p: f64) -> i32 {
    f64::ceil(f64::sqrt(2.0 * 365.0 * f64::ln(1.0 / (1.0 - p)))) as i32
}

fn f_filled(p: f64) -> f64 {
    // Implement the function logic here
    0.0
}

fn main() {
    let param0 = [
        0.9303713975220877,
        0.48126843587453595,
        0.48776789524757905,
        0.35184405927337793,
        0.8000415444743662,
        0.3528645948885943,
        0.33594265260473667,
        0.3603861267753616,
        7218.247044923335,
        -4701.904717953173,
    ];

    for &p in param0.iter() {
        f_gold(p);
    }
}

#[cfg(kani)]
#[kani::proof]
fn foo() {
    main()
}