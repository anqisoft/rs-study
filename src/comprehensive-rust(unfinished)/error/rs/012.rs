// error\012.rs
// https://google.github.io/comprehensive-rust/zh-CN/ownership/move-semantics.html

fn main() {
    let s1: String = String::from("Hello!");
    let s2: String = s1;
    println!("s2: {s2}");
    println!("s1: {s1}");
}

/* result:
error[E0382]: borrow of moved value: `s1`
 --> ..\rs\012.rs:8:19
  |
5 |     let s1: String = String::from("Hello!");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
6 |     let s2: String = s1;
  |                      -- value moved here
7 |     println!("s2: {s2}");
8 |     println!("s1: {s1}");
  |                   ^^^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
6 |     let s2: String = s1.clone();
  |                        ++++++++

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
*/
