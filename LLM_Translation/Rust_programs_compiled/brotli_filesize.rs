type Int64 = i64;
use std::fs::File;
use std::io::{self, Seek, SeekFrom};

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

fn main() {
}