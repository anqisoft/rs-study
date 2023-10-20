// 063.rs
// https://www.runoob.com/rust/rust-file-io.html

use std::fs;

fn main() {
    fs::write("D:\\text.txt", "FROM RUST PROGRAM")
        .unwrap();
}

/* result: (empty)

*/
