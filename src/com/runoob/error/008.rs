// error/008.rs
// https://www.runoob.com/rust/rust-conditions.html

fn main() {
    let number = 3;
    if number {
        println!("Yes");
    }
}

/* result:
error[E0308]: mismatched types
 --> ..\008.rs:6:8
  |
6 |     if number {
  |        ^^^^^^ expected `bool`, found integer

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
*/
