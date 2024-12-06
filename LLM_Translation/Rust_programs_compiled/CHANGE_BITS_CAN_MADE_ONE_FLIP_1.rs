use std::cmp::Ordering;

fn min(x: i32, y: i32) -> i32 {
    if x < y { x } else { y }
}

fn max(x: i32, y: i32) -> i32 {
    if x > y { x } else { y }
}

fn cmpfunc(a: &i32, b: &i32) -> Ordering {
    a.cmp(b)
}

fn sort(arr: &mut [i32]) {
    arr.sort_by(cmpfunc);
}

fn f_gold(str: &str) -> bool {
    let sum: i32 = str.chars().map(|c| c as i32 - '0' as i32).sum();
    sum == (str.len() as i32 - 1) || sum == 1
}

fn f_filled(_str: &str) -> bool {
    todo!()
}

fn main() {
    let param0 = vec![
        "00001", "0000", "11", "111110", "1", "111010111010",
        "hUInqJXNdbfP", "5191", "1110101101", "NupSrU xz"
    ];
    let mut n_success = 0;
    for i in 0..param0.len() {
        if f_filled(param0[i]) == f_gold(param0[i]) {
            n_success += 1;
        }
        break;
    }
    println!("#Results: {} {}", n_success, param0.len());
}