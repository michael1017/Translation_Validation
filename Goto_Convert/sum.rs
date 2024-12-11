#[no_mangle]
fn sum_rs(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let a: i32 = 1;
    let b: i32 = 2;;

    sum_rs(a, b);
}

#[cfg(kani)]
#[kani::proof]
fn foo() {
    main()
}
