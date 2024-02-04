// 089.rs
// https://google.github.io/comprehensive-rust/zh-CN/generics/monomorphization.html

fn test1() {
    println!("In test1()");
    let integer = Some(5);
    let float = Some(5.0);

    // error[E0277]: `Option<{integer}>` doesn't implement `std::fmt::Display`--> ..\rs\089.rs:9:26
    // `Option<{integer}>` cannot be formatted with the default formatter
    //   = help: the trait `std::fmt::Display` is not implemented for `Option<{integer}>`
    //   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    //   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
    // error[E0277]: `Option<{float}>` doesn't implement `std::fmt::Display`
    //  --> ..\rs\089.rs:9:46
    //   |
    // 9 |     println!("integer is {integer}, float is {float}");
    //   |                                              ^^^^^^^ `Option<{float}>` cannot be formatted with the default formatter
    //   |
    //   = help: the trait `std::fmt::Display` is not implemented for `Option<{float}>`
    //   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    //   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
    // println!("integer is {integer}, float is {float}");

    println!("integer is {integer:?}, float is {float:?}");
}

fn test2() {
    println!("\nIn test2()");

    #[derive(Debug)]
    enum Option_i32 {
        Some(i32),
        None,
    }

    #[derive(Debug)]
    enum Option_f64 {
        Some(f64),
        None,
    }
    
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
    // error[E0277]: `Option_i32` doesn't implement `std::fmt::Display`
    //   --> ..\rs\089.rs:42:26
    //   |
    // 42 |     println!("integer is {integer}, float is {float}");
    //   |                          ^^^^^^^^^ `Option_i32` cannot be formatted with the default formatter
    //   |
    //   = help: the trait `std::fmt::Display` is not implemented for `Option_i32`
    //   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    //   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
    // error[E0277]: `Option_f64` doesn't implement `std::fmt::Display`
    //   --> ..\rs\089.rs:42:46
    //   |
    // 42 |     println!("integer is {integer}, float is {float}");
    //   |                                              ^^^^^^^ `Option_f64` cannot be formatted with the default formatter
    //   |
    //   = help: the trait `std::fmt::Display` is not implemented for `Option_f64`
    //   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    //   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
    // error: aborting due to 2 previous errors; 2 warnings emitted
    // println!("integer is {integer}, float is {float}");

    println!("integer is {integer:#?}, float is {float:#?}");
}

fn main() {
    test1();
    test2();
}

/* result:
In test1()
integer is Some(5), float is Some(5.0)

In test2()
integer is Some(
    5,
), float is Some(
    5.0,
)
*/

/*
warning: type `Option_i32` should have an upper camel case name
  --> ..\rs\089.rs:32:10
   |
32 |     enum Option_i32 {
   |          ^^^^^^^^^^ help: convert the identifier to upper camel case: `OptionI32`
   |
   = note: `#[warn(non_camel_case_types)]` on by default

warning: type `Option_f64` should have an upper camel case name
  --> ..\rs\089.rs:38:10
   |
38 |     enum Option_f64 {
   |          ^^^^^^^^^^ help: convert the identifier to upper camel case: `OptionF64`

warning: variant `None` is never constructed
  --> ..\rs\089.rs:34:9
   |
32 |     enum Option_i32 {
   |          ---------- variant in this enum
33 |         Some(i32),
34 |         None,
   |         ^^^^
   |
   = note: `Option_i32` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` on by default

warning: variant `None` is never constructed
  --> ..\rs\089.rs:40:9
   |
38 |     enum Option_f64 {
   |          ---------- variant in this enum
39 |         Some(f64),
40 |         None,
   |         ^^^^
   |
   = note: `Option_f64` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis

warning: 4 warnings emitted
*/