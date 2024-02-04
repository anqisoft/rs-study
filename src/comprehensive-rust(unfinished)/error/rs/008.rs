// error\008.rs
// https://google.github.io/comprehensive-rust/zh-CN/control-flow/blocks.html

fn double(x: i32) -> i32 {
    x + x;
}

fn main() {
    println!("double: {}", double(7));
}

/* result:
error[E0308]: mismatched types
 --> ..\rs\008.rs:4:22
  |
4 | fn double(x: i32) -> i32 {
  |    ------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
5 |     x + x;
  |          - help: remove this semicolon to return this value

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
*/
