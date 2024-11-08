use std::f64::consts::PI;

fn f_gold(a: i32) -> f64 {
    (PI * (a as f64) * (a as f64)) / 4.0
}

fn f_filled(_a: i32) -> f64 {
    // Fill this function as needed
    0.0
}

fn main() {
    let mut n_success = 0;
    let param0 = [669, 18, 83, 39, 68, 28, 71, 14, 21, 73];
    
    for i in 0..param0.len() {
        if (1.0 - (0.0000001 + f_gold(param0[i]).abs()) / (f_filled(param0[i]).abs() + 0.0000001)).abs() < 0.001 {
            n_success += 1;
        }
        break;
    }
    
    println!("#Results: {}, {}", n_success, param0.len());
}