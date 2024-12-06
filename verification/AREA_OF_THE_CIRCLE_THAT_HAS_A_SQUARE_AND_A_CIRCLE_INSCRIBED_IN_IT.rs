use std::f64::consts::PI;

#[no_mangle]
fn f_gold_r(a: i32) -> f32 {
    ((PI as f32) * (a as f32) * (a as f32)) / 4.0
}


#[cfg(kani)]
#[kani::proof]
fn main_rs() {
    f_gold_r(0);
}