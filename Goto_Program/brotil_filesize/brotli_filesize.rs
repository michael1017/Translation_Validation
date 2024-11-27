type Int64 = i64;
use std::fs::File;
use std::io::{self, Seek, SeekFrom};

#[no_mangle]
fn file_size(path: &str) -> Int64 {
    match File::open(path) {
        Ok(mut f) => {
            if f.seek(SeekFrom::End(0)).is_ok() {
                match f.stream_position() {
                    Ok(pos) => pos as Int64,
                    Err(_) => -1,
                }
            } else {
                -1
            }
        },
        Err(_) => -1,
    }
}

#[no_mangle]
fn rs_main() {
    let path = "example.txt"; // Replace with the actual file path

    // Call the file_size function
    let size = file_size(path);

    // Check the result and print an appropriate message
    if size >= 0 {
        println!("The size of the file '{}' is {} bytes.", path, size);
    } else {
        println!("Failed to determine the size of the file '{}'.", path);
    }
}

#[no_mangle]
#[cfg(kani)]
#[kani::proof]
fn foo() {
    rs_main()
}