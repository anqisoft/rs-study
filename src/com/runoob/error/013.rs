// error/013.rs
// https://www.runoob.com/rust/rust-slice.html

fn main() {
    let mut s = String::from("runoob");
    let slice = &s[0..3];
    s.push_str("yes!"); // 错误
    println!("slice = {}", slice);
}

/* result:
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> ..\013.rs:7:5
  |
6 |     let slice = &s[0..3];
  |                  - immutable borrow occurs here
7 |     s.push_str("yes!"); // 错误
  |     ^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
8 |     println!("slice = {}", slice);
  |                            ----- immutable borrow later used here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0502`.
*/