// 060.rs
// https://www.runoob.com/rust/rust-file-io.html

use std::fs;

fn main() {
    let text = fs::read_to_string("./text.txt").unwrap();
    println!("{}", text);
}

/* result:
thread 'main' panicked at ..\060.rs:7:47:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/
