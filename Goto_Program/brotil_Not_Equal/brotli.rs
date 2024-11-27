const BROTLI_TRUE: i32 = 1;
const BROTLI_FALSE: i32 = 0;

#[no_mangle]
fn parse_int(s: &str, low: i32, high: i32) -> i32 {
    let mut value = 0;
    let mut chars = s.chars();
    
    for i in 0..5 {
        if let Some(c) = chars.next() {
            if c < '0' || c > '9' {
                return -1;
            }
            value = (10 * value) + (c as i32 - '0' as i32);
        } else {
            break;
        }
    }
    
    if s.len() == 0 || (s.len() > 1 && s.starts_with('0')) || value < low || value > high {
        return -1;
    }
    
    value
}

#[cfg(kani)]
#[kani::proof]
fn rs_main() {
    // The main function is currently empty.
    // You can add code here to test the `parse_int` function.
    let input = "123";

    parse_int(input, 0, 200);
}