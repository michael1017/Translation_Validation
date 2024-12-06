use std::cmp::Ordering;

#[no_mangle]
fn utoa_rust(v: usize) -> String {
    // Create a buffer to store the digits (Rust uses String for dynamic strings)
    let mut buffer = Vec::new();
    
    // Convert the number to digits in reverse order
    let mut value = v;
    loop {
        buffer.push((value % 10) as u8 + b'0'); // Add the ASCII value of the digit
        value /= 10;
        if value == 0 {
            break;
        }
    }
    
    // Reverse the buffer to get the correct order
    buffer.reverse();
    
    // Convert the buffer to a String and return
    String::from_utf8(buffer).expect("Conversion to String failed")
}
// fn utoa_rust(mut v: usize) -> String {
//     let mut str = String::new();
//     loop {
//         str.push(((v % 10) as u8 + b'0') as char);
//         v /= 10;
//         if v == 0 { break; }
//     }
//     str.chars().rev().collect()
// }

#[no_mangle]
fn is_ascending_order_rust(v: &[u64]) -> bool {
    for j in 1..v.len() {
        if v[j - 1] >= v[j] {
            return false;
        }
    }
    true
}

#[cfg(kani)]
#[kani::proof]
fn mainr() {
    // Create input and call is_ascending_order_rust
    let input = [1, 2, 3, 4, 5];
    let _result = is_ascending_order_rust(&input);

    // Create input and call utoa_rust
    let input = 12345;
    let _result = utoa_rust(input);
}