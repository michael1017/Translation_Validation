fn sum2(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(kani)]
#[kani::proof]
fn kani_harness() {
    // Create symbolic (non-deterministic) values for a and b
    let a: i32 = kani::any();
    let b: i32 = kani::any();

    // Call the sum function with these symbolic values
    sum2(a, b);
}