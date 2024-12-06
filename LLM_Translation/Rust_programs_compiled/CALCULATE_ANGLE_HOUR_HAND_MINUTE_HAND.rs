// Copyright (c) 2019-present, Facebook, Inc.
// All rights reserved.
//
// This source code is licensed under the license found in the
// LICENSE file in the root directory of this source tree.
//

use std::cmp;

fn min(x: i32, y: i32) -> i32 {
    if x < y { x } else { y }
}

fn max(x: i32, y: i32) -> i32 {
    if x > y { x } else { y }
}

fn cmpfunc(a: &i32, b: &i32) -> cmp::Ordering {
    a.cmp(b)
}

fn sort(arr: &mut [i32]) {
    arr.sort_by(cmpfunc);
}

fn f_gold(h: f64, m: f64) -> i32 {
    if h < 0.0 || m < 0.0 || h > 12.0 || m > 60.0 {
        println!("Wrong input");
    }
    let mut h = if h == 12.0 { 0.0 } else { h };
    let mut m = if m == 60.0 { 0.0 } else { m };
    let hour_angle = (0.5 * (h * 60.0 + m)).round() as i32;
    let minute_angle = (6.0 * m).round() as i32;
    let angle = (hour_angle - minute_angle).abs();
    min(360 - angle, angle)
}

fn f_filled(_h: f64, _m: f64) -> i32 {
    // Fill in the function body
    0
}

fn main() {
    let mut n_success = 0;
    let param0 = [7322.337365895532, -0.5025472034247969, 8735.336068205026, -5478.862697905712, 8264.126919165505, -9671.311773842834, 9995.328351000411, -5274.574323066984, 1310.8711644223736, -2829.678131972794];
    let param1 = [6996.326968156217, -2910.070017192333, 1910.3752934680874, -9470.18148108585, 7058.937313484608, -3867.070379361206, 2145.339179488316, -3583.7503371694124, 5214.059687285893, -9371.556600288217];

    for i in 0..param0.len() {
        if f_filled(param0[i], param1[i]) == f_gold(param0[i], param1[i]) {
            n_success += 1;
        }
        break;
    }
    println!("#Results: {} {}", n_success, param0.len());
}