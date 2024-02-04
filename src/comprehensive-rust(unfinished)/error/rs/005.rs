// error\005.rs
// https://google.github.io/comprehensive-rust/zh-CN/exercises/day-1/implicit-conversions.html

fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    test1();
    test2();
    test3();
    test4();
    test5();

    test6();
    test7();
    test8();

    test9();
    test10();
}

fn test1() {
    println!("In test1()");

    let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x, y));
    // println!("{x} * {y} = {}", multiply(x.into(), y));
}

fn test2() {
    println!("\nIn test2()");

    let x: f32 = 15.0;
    let y: f64 = 1000.0;

    println!("{x} * {y} = {}", multiply(x, y));
}

fn test3() {
    println!("\nIn test3()");

    let x: i32 = 15;
    let y: i64 = 1000;

    println!("{x} * {y} = {}", multiply(x, y));
}

fn test4() {
    println!("\nIn test4()");

    let x: i128 = 15;
    let y: isize = 1000;

    println!("{x} * {y} = {}", multiply(x, y));
}

fn test5() {
    println!("\nIn test5()");

    let x: bool = true;
    let y: bool = false;

    println!("{x} * {y} = {}", multiply(x, y));
}

fn test6() {
    println!("In test6()");

    let x: u8 = 15;
    let y: u16 = 1000;

    println!("{x} * {y} = {}", multiply(x, y));
    // println!("{x} * {y} = {}", multiply(x.into(), y.into()));
}

fn test7() {
    println!("\nIn test7()");

    let x: u32 = 15;
    let y: u64 = 1000;

    println!("{x} * {y} = {}", multiply(x, y));
}

fn test8() {
    println!("\nIn test8()");

    let x: u128 = 15;
    let y: usize = 1000;

    println!("{x} * {y} = {}", multiply(x, y));
}

fn test9() {
    println!("\nIn test9()");

    let x: String = String::from("15");
    let y: String = String::from("1000");

    println!("{x} * {y} = {}", multiply(x, y));
}

fn test10() {
    println!("\nIn test10()");

    let x: &str = "15";
    let y: &str = "1000";

    println!("{x} * {y} = {}", multiply(x, y));
}

/* errors
error[E0308]: mismatched types
  --> ..\rs\005.rs:29:41
   |
29 |     println!("{x} * {y} = {}", multiply(x, y));
   |                                -------- ^ expected `i16`, found `i8`
   |                                |
   |                                arguments to this function are incorrect
   |
note: function defined here
  --> ..\rs\005.rs:4:4
   |
4  | fn multiply(x: i16, y: i16) -> i16 {
   |    ^^^^^^^^ ------
help: you can convert an `i8` to an `i16`
   |
29 |     println!("{x} * {y} = {}", multiply(x.into(), y));
   |                                          +++++++

error[E0308]: arguments to this function are incorrect
  --> ..\rs\005.rs:39:32
   |
39 |     println!("{x} * {y} = {}", multiply(x, y));
   |                                ^^^^^^^^ -  - expected `i16`, found `f64`
   |                                         |
   |                                         expected `i16`, found `f32`
   |
note: function defined here
  --> ..\rs\005.rs:4:4
   |
4  | fn multiply(x: i16, y: i16) -> i16 {
   |    ^^^^^^^^ ------  ------

error[E0308]: arguments to this function are incorrect
  --> ..\rs\005.rs:48:32
   |
48 |     println!("{x} * {y} = {}", multiply(x, y));
   |                                ^^^^^^^^ -  - expected `i16`, found `i64`
   |                                         |
   |                                         expected `i16`, found `i32`
   |
note: function defined here
  --> ..\rs\005.rs:4:4
   |
4  | fn multiply(x: i16, y: i16) -> i16 {
   |    ^^^^^^^^ ------  ------
help: you can convert an `i32` to an `i16` and panic if the converted value doesn't fit
   |
48 |     println!("{x} * {y} = {}", multiply(x.try_into().unwrap(), y));
   |                                          ++++++++++++++++++++
help: you can convert an `i64` to an `i16` and panic if the converted value doesn't fit
   |
48 |     println!("{x} * {y} = {}", multiply(x, y.try_into().unwrap()));
   |                                             ++++++++++++++++++++

error[E0308]: arguments to this function are incorrect
  --> ..\rs\005.rs:57:32
   |
57 |     println!("{x} * {y} = {}", multiply(x, y));
   |                                ^^^^^^^^ -  - expected `i16`, found `isize`
   |                                         |
   |                                         expected `i16`, found `i128`
   |
note: function defined here
  --> ..\rs\005.rs:4:4
   |
4  | fn multiply(x: i16, y: i16) -> i16 {
   |    ^^^^^^^^ ------  ------
help: you can convert an `i128` to an `i16` and panic if the converted value doesn't fit
   |
57 |     println!("{x} * {y} = {}", multiply(x.try_into().unwrap(), y));
   |                                          ++++++++++++++++++++
help: you can convert an `isize` to an `i16` and panic if the converted value doesn't fit
   |
57 |     println!("{x} * {y} = {}", multiply(x, y.try_into().unwrap()));
   |                                             ++++++++++++++++++++

error[E0308]: arguments to this function are incorrect
  --> ..\rs\005.rs:66:32
   |
66 |     println!("{x} * {y} = {}", multiply(x, y));
   |                                ^^^^^^^^ -  - expected `i16`, found `bool`
   |                                         |
   |                                         expected `i16`, found `bool`
   |
note: function defined here
  --> ..\rs\005.rs:4:4
   |
4  | fn multiply(x: i16, y: i16) -> i16 {
   |    ^^^^^^^^ ------  ------

error[E0308]: arguments to this function are incorrect
  --> ..\rs\005.rs:75:32
   |
75 |     println!("{x} * {y} = {}", multiply(x, y));
   |                                ^^^^^^^^ -  - expected `i16`, found `u16`
   |                                         |
   |                                         expected `i16`, found `u8`
   |
note: function defined here
  --> ..\rs\005.rs:4:4
   |
4  | fn multiply(x: i16, y: i16) -> i16 {
   |    ^^^^^^^^ ------  ------
help: you can convert a `u8` to an `i16`
   |
75 |     println!("{x} * {y} = {}", multiply(x.into(), y));
   |                                          +++++++
help: you can convert a `u16` to an `i16` and panic if the converted value doesn't fit
   |
75 |     println!("{x} * {y} = {}", multiply(x, y.try_into().unwrap()));
   |                                             ++++++++++++++++++++

error[E0308]: arguments to this function are incorrect
  --> ..\rs\005.rs:85:32
   |
85 |     println!("{x} * {y} = {}", multiply(x, y));
   |                                ^^^^^^^^ -  - expected `i16`, found `u64`
   |                                         |
   |                                         expected `i16`, found `u32`
   |
note: function defined here
  --> ..\rs\005.rs:4:4
   |
4  | fn multiply(x: i16, y: i16) -> i16 {
   |    ^^^^^^^^ ------  ------
help: you can convert a `u32` to an `i16` and panic if the converted value doesn't fit
   |
85 |     println!("{x} * {y} = {}", multiply(x.try_into().unwrap(), y));
   |                                          ++++++++++++++++++++
help: you can convert a `u64` to an `i16` and panic if the converted value doesn't fit
   |
85 |     println!("{x} * {y} = {}", multiply(x, y.try_into().unwrap()));
   |                                             ++++++++++++++++++++

error[E0308]: arguments to this function are incorrect
  --> ..\rs\005.rs:94:32
   |
94 |     println!("{x} * {y} = {}", multiply(x, y));
   |                                ^^^^^^^^ -  - expected `i16`, found `usize`
   |                                         |
   |                                         expected `i16`, found `u128`
   |
note: function defined here
  --> ..\rs\005.rs:4:4
   |
4  | fn multiply(x: i16, y: i16) -> i16 {
   |    ^^^^^^^^ ------  ------
help: you can convert a `u128` to an `i16` and panic if the converted value doesn't fit
   |
94 |     println!("{x} * {y} = {}", multiply(x.try_into().unwrap(), y));
   |                                          ++++++++++++++++++++
help: you can convert a `usize` to an `i16` and panic if the converted value doesn't fit
   |
94 |     println!("{x} * {y} = {}", multiply(x, y.try_into().unwrap()));
   |                                             ++++++++++++++++++++

error[E0308]: arguments to this function are incorrect
   --> ..\rs\005.rs:103:32
    |
103 |     println!("{x} * {y} = {}", multiply(x, y));
    |                                ^^^^^^^^ -  - expected `i16`, found `String`
    |                                         |
    |                                         expected `i16`, found `String`
    |
note: function defined here
   --> ..\rs\005.rs:4:4
    |
4   | fn multiply(x: i16, y: i16) -> i16 {
    |    ^^^^^^^^ ------  ------

error[E0308]: arguments to this function are incorrect
   --> ..\rs\005.rs:112:32
    |
112 |     println!("{x} * {y} = {}", multiply(x, y));
    |                                ^^^^^^^^ -  - expected `i16`, found `&str`
    |                                         |
    |                                         expected `i16`, found `&str`
    |
note: function defined here
   --> ..\rs\005.rs:4:4
    |
4   | fn multiply(x: i16, y: i16) -> i16 {
    |    ^^^^^^^^ ------  ------

error: aborting due to 10 previous errors

For more information about this error, try `rustc --explain E0308`.
*/