
fn f_gold(a: &[i32], n: usize, k: usize) -> i32 {
    if k >= n - 1 {
        return n as i32;
    }
    
    let mut best = 0;
    let mut times = 0;
    
    for i in 0..n {
        if a[i] > best {
            best = a[i];
            if i > 0 {
                times = 1;
            }
        } else {
            times += 1;
        }
        if times >= k {
            return best;
        }
    }
    best
}

#[cfg(kani)]
#[kani::proof]
fn main() {
    let param0_3 = [90,23,43,42,7];
    f_gold(&param0_3, 4, 4);
}
