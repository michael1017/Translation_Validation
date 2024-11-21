use std::f64::consts::PI;

fn f_gold(a: i32) -> f32 {
    ((3.142 as f32) * (a as f32) * (a as f32)) / 4.0
}


#[cfg(kani)]
#[kani::proof]
fn main_rs() {
    f_gold(0);
}