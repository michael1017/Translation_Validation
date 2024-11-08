use std::cmp::Ordering;

fn utoa(mut v: usize) -> String {
    let mut str = String::new();
    loop {
        str.push(((v % 10) as u8 + b'0') as char);
        v /= 10;
        if v == 0 { break; }
    }
    str.chars().rev().collect()
}

fn is_ascending_order(v: &[u64]) -> bool {
    v.windows(2).all(|w| w[0].cmp(&w[1]) == Ordering::Less)
}

fn main() {}