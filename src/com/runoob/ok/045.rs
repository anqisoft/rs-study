// 045.rs
// https://www.runoob.com/rust/rust-error-handle.html

use std::fs::File;

fn main() {
    let f1 = File::open("hello.txt").unwrap();
    let f2 = File::open("hello.txt").expect("Failed to open.");
}

/* result:
thread 'main' panicked at ..\045.rs:7:38:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/

/* warnings:
warning: unused variable: `f1`
 --> ..\045.rs:7:9
  |
7 |     let f1 = File::open("hello.txt").unwrap();
  |         ^^ help: if this is intentional, prefix it with an underscore: `_f1`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `f2`
 --> ..\045.rs:8:9
  |
8 |     let f2 = File::open("hello.txt").expect("Failed to open.");
  |         ^^ help: if this is intentional, prefix it with an underscore: `_f2`

warning: 2 warnings emitted
*/
