

fn f_gold(a: i32, b: i32) -> i32 {
    if a == 0 {
        return b;
    }
    f_gold(b % a, a)
}

#[cfg(kani)]
#[kani::proof]
fn main() {
    f_gold(0, 0);
}