// 044.rs
// https://www.runoob.com/rust/rust-error-handle.html

/*
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    match f {
        Ok(file) => {
            println!("File opened successfully.");
        },
        Err(err) => {
            println!("Failed to open the file.");
        }
    }
}

/* result:
Failed to open the file.
*/

/* warnings:
warning: unused variable: `file`
  --> ..\044.rs:16:12
   |
16 |         Ok(file) => {
   |            ^^^^ help: if this is intentional, prefix it with an underscore: `_file`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `err`
  --> ..\044.rs:19:13
   |
19 |         Err(err) => {
   |             ^^^ help: if this is intentional, prefix it with an underscore: `_err`

warning: 2 warnings emitted
*/
