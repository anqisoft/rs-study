// 061.rs
// https://www.runoob.com/rust/rust-file-io.html

use std::fs;

fn main() {
    let content = fs::read("text.txt").unwrap();
    println!("{:?}", content);
}

/* result:
thread 'main' panicked at ..\061.rs:7:40:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/
