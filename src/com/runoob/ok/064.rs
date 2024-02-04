// 064.rs
// https://www.runoob.com/rust/rust-file-io.html

use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut file = File::create("text.txt").unwrap();
    file.write(b"FROM RUST PROGRAM").unwrap();
}

/* result: (empty)

*/
