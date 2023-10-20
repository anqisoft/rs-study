// error/024.rs
// https://www.runoob.com/rust/rust-concurrency.html
// The right file is ok/081.rs

use std::thread;

fn main() {
    let s = "hello";

    let handle = thread::spawn(|| {
        println!("{}", s);
    });

    handle.join().unwrap();
}

/* result:
error[E0373]: closure may outlive the current function, but it borrows `s`, which is owned by the current function
  --> ..\024.rs:10:32
   |
10 |     let handle = thread::spawn(|| {
   |                                ^^ may outlive borrowed value `s`
11 |         println!("{}", s);
   |                        - `s` is borrowed here
   |
note: function requires argument type to outlive `'static`
  --> ..\024.rs:10:18
   |
10 |       let handle = thread::spawn(|| {
   |  __________________^
11 | |         println!("{}", s);
12 | |     });
   | |______^
help: to force the closure to take ownership of `s` (and any other referenced variables), use the `move` keyword
   |
10 |     let handle = thread::spawn(move || {
   |                                ++++

error: aborting due to previous error

For more information about this error, try `rustc --explain E0373`.
*/
