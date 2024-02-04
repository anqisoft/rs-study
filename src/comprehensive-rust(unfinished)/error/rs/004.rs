// error\004.rs
// https://google.github.io/comprehensive-rust/zh-CN/exercises/day-1/implicit-conversions.html

fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x, y));
}

/* result:
error[E0308]: mismatched types
  --> ..\rs\004.rs:12:41
   |
12 |     println!("{x} * {y} = {}", multiply(x, y));
   |                                -------- ^ expected `i16`, found `i8`
   |                                |
   |                                arguments to this function are incorrect
   |
note: function defined here
  --> ..\rs\004.rs:4:4
   |
4  | fn multiply(x: i16, y: i16) -> i16 {
   |    ^^^^^^^^ ------
help: you can convert an `i8` to an `i16`
   |
12 |     println!("{x} * {y} = {}", multiply(x.into(), y));
   |                                          +++++++

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
*/
