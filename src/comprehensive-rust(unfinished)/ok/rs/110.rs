// 110.rs
// https://google.github.io/comprehensive-rust/zh-CN/error-handling/result.html

use std::fs;
use std::io::Read;

fn main() {
    let file = fs::File::open("diary.txt");
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents);
            println!("Dear diary: {contents}");
        },
        Err(err) => {
            println!("The diary could not be opened: {err}");
        }
    }
}

/* result:
The diary could not be opened: The system cannot find the file specified. (os error 2)
*/

/*
warning: unused `Result` that must be used
  --> ..\rs\110.rs:12:13
   |
12 |             file.read_to_string(&mut contents);
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this `Result` may be an `Err` variant, which should be handled
   = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
   |
12 |             let _ = file.read_to_string(&mut contents);
   |             +++++++

warning: 1 warning emitted
*/