fn another_function_p69() {
    println!("Another function.");
}
/*
Another function.
*/

fn another_function_p70(x: i32) {
    println!("The value of x is: {x}");
}
/*
The value of x is: 5
*/

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
/*
The measurement is: 5h
*/

fn error1() {
    // Syntax Error: `let` expressions are not supported here   rust-analyzer(syntax-error)
    // let x = (let y = 6);
}
/*
error: expected expression, found `let` statement
  --> src\main.rs:24:14
   |
24 |     let x = (let y = 6);
   |              ^^^

error: expected expression, found statement (`let`)
  --> src\main.rs:24:14
   |
24 |     let x = (let y = 6);
   |              ^^^^^^^^^
   |
   = note: variable declaration using `let` is a statement

error[E0658]: `let` expressions in this position are unstable
  --> src\main.rs:24:14
   |
24 |     let x = (let y = 6);
   |              ^^^^^^^^^
   |
   = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information

warning: unnecessary parentheses around assigned value
  --> src\main.rs:24:13
   |
24 |     let x = (let y = 6);
   |             ^         ^
   |
   = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
   |
24 -     let x = (let y = 6);
24 +     let x = let y = 6;
   |

For more information about this error, try `rustc --explain E0658`.
*/

fn error2() {
    // let x = let y = 6;
}
/*
error: expected expression, found `let` statement
  --> src\main.rs:66:13
   |
66 |     let x = let y = 6;
   |             ^^^

error: expected expression, found statement (`let`)
  --> src\main.rs:66:13
   |
66 |     let x = let y = 6;
   |             ^^^^^^^^^
   |
   = note: variable declaration using `let` is a statement

error[E0658]: `let` expressions in this position are unstable
  --> src\main.rs:66:13
   |
66 |     let x = let y = 6;
   |             ^^^^^^^^^
   |
   = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information

For more information about this error, try `rustc --explain E0658`.
*/

fn error3() {
    // let x = y = 6;
}
/*
error[E0425]: cannot find value `y` in this scope
  --> src\main.rs:95:13
   |
95 |     let x = y = 6;
   |             ^
   |
help: you might have meant to introduce a new binding
   |
95 |     let x = let y = 6;
   |             +++

For more information about this error, try `rustc --explain E0425`.
*/

fn error4() {
    // let mut y = 5;
    // let x = y = 6;
    // println!("x is {x}, y is {y}");
}
/*
error[E0277]: `()` doesn't implement `std::fmt::Display`
   --> src\main.rs:115:20
    |
115 |     println!("x is {x}, y is {y}");
    |                    ^^^ `()` cannot be formatted with the default formatter
    |
    = help: the trait `std::fmt::Display` is not implemented for `()`
    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
*/

fn test1() {
    println!("\nIn test1()");
    let mut y = 5;
    let x = y = 6;
    println!("x is {x:?}, y is {y}");
}
/*
In test1()
x is (), y is 6
*/
/*
warning: value assigned to `y` is never read
   --> src\main.rs:133:13
    |
133 |     let mut y = 5;
    |             ^
    |
    = help: maybe it is overwritten before being read?
    = note: `#[warn(unused_assignments)]` on by default

warning: `functions` (bin "functions") generated 1 warning
*/

fn test2() {
    println!("\nIn test2()");
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}
/*
In test2()
The value of y is: 4
*/

// P72
fn five() -> i32 {
    5
}

// P73
fn plus_one(x: i32) -> i32 {
    x + 1
}

// P73
// fn plus_one_error(x: i32) -> i32 {
//     x + 1;
// }
/*
error[E0308]: mismatched types
   --> src\main.rs:178:30
    |
178 | fn plus_one_error(x: i32) -> i32 {
    |    --------------            ^^^ expected `i32`, found `()`
    |    |
    |    implicitly returns `()` as its body has no tail or `return` expression
179 |     x + 1;
    |          - help: remove this semicolon to return this value

For more information about this error, try `rustc --explain E0308`.
*/

fn main() {
    println!("Hello, world!");

    another_function_p69();
    another_function_p70(5);

    print_labeled_measurement(5, 'h');

    error1();
    error2();
    error3();
    error4();

    test1();
    test2();

    let x = five();
    println!("The value of x is: {x}");
    // The value of x is: 5

    let x = plus_one(5);
    println!("The value of x is: {x}");
    // The value of x is: 6
}
/*
Hello, world!
*/
