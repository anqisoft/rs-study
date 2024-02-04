// 113.rs
// https://google.github.io/comprehensive-rust/zh-CN/error-handling/deriving-error-enums.html

// See: example_113\src\main.rs
// I don't know how to debug it directly. So I converted it to a new project and debugged it.

extern crate thiserror;
use std::{fs, io};
use std::io::Read;
use thiserror::Error;

#[derive(Debug, Error)]
enum ReadUsernameError {
    #[error("Could not read: {0}")]
    IoError(#[from] io::Error),
    #[error("Found no username in {0}")]
    EmptyUsername(String),
}

fn read_username(path: &str) -> Result<String, ReadUsernameError> {
    let mut username = String::new();
    fs::File::open(path)?.read_to_string(&mut username)?;
    if username.is_empty() {
        return Err(ReadUsernameError::EmptyUsername(String::from(path)));
    }
    Ok(username)
}

// fn main() {
//     //fs::write("config.dat", "").unwrap();
//     match read_username("config.dat") {
//         Ok(username) => println!("Username: {username}"),
//         Err(err)     => println!("Error: {err}"),
//     }
// }

fn main() {
    test1();
    test2();
    test3();
}

const FILE_NAME: &str = "config.dat";
fn test1() {
    println!("In test1()");
    use std::path::Path;
    if Path::new(FILE_NAME).exists() {
        fs::remove_file(FILE_NAME).unwrap();
    }

    //fs::write(FILE_NAME, "").unwrap();
    match read_username(FILE_NAME) {
        Ok(username) => println!("Username: {username}"),
        Err(err)     => println!("Error: {err}"),
    }
}

fn test2() {
    println!("\nIn test2()");
    fs::write(FILE_NAME, "").unwrap();

    match read_username(FILE_NAME) {
        Ok(username) => println!("Username: {username}"),
        Err(err)     => println!("Error: {err}"),
    }
}

fn test3() {
    println!("\nIn test3()");
    fs::write(FILE_NAME, "alice").unwrap();

    match read_username(FILE_NAME) {
        Ok(username) => println!("Username: {username}"),
        Err(err)     => println!("Error: {err}"),
    }
}

/* result:

*/

/*
error[E0432]: unresolved import `thiserror`
 --> ..\rs\113.rs:6:5
  |
6 | use thiserror::Error;
  |     ^^^^^^^^^ maybe a missing crate `thiserror`?
  |
  = help: consider adding `extern crate thiserror` to use the `thiserror` crate

error: cannot determine resolution for the derive macro `Error`
 --> ..\rs\113.rs:8:17
  |
8 | #[derive(Debug, Error)]
  |                 ^^^^^
  |
  = note: import resolution is stuck, try simplifying macro imports

error: cannot find attribute `error` in this scope
  --> ..\rs\113.rs:10:7
   |
10 |     #[error("Could not read: {0}")]
   |       ^^^^^

error: cannot find attribute `from` in this scope
  --> ..\rs\113.rs:11:15
   |
11 |     IoError(#[from] io::Error),
   |               ^^^^

error: cannot find attribute `error` in this scope
  --> ..\rs\113.rs:12:7
   |
12 |     #[error("Found no username in {0}")]
   |       ^^^^^

error[E0277]: `?` couldn't convert the error to `ReadUsernameError`
  --> ..\rs\113.rs:18:25
   |
16 | fn read_username(path: &str) -> Result<String, ReadUsernameError> {
   |                                 --------------------------------- expected `ReadUsernameError` because of this
17 |     let mut username = String::new();
18 |     fs::File::open(path)?.read_to_string(&mut username)?;
   |                         ^ the trait `From<std::io::Error>` is not implemented for `ReadUsernameError`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: the following other types implement trait `FromResidual<R>`:
             <Result<T, F> as FromResidual<Yeet<E>>>
             <Result<T, F> as FromResidual<Result<Infallible, E>>>
   = note: required for `Result<String, ReadUsernameError>` to implement `FromResidual<Result<Infallible, std::io::Error>>`

error[E0277]: `?` couldn't convert the error to `ReadUsernameError`
  --> ..\rs\113.rs:18:56
   |
16 | fn read_username(path: &str) -> Result<String, ReadUsernameError> {
   |                                 --------------------------------- expected `ReadUsernameError` because of this
17 |     let mut username = String::new();
18 |     fs::File::open(path)?.read_to_string(&mut username)?;
   |                                                        ^ the trait `From<std::io::Error>` is not implemented for `ReadUsernameError`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: the following other types implement trait `FromResidual<R>`:
             <Result<T, F> as FromResidual<Yeet<E>>>
             <Result<T, F> as FromResidual<Result<Infallible, E>>>
   = note: required for `Result<String, ReadUsernameError>` to implement `FromResidual<Result<Infallible, std::io::Error>>`

error[E0277]: `ReadUsernameError` doesn't implement `std::fmt::Display`
  --> ..\rs\113.rs:50:42
   |
50 |         Err(err)     => println!("Error: {err}"),
   |                                          ^^^^^ `ReadUsernameError` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `ReadUsernameError`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `ReadUsernameError` doesn't implement `std::fmt::Display`
  --> ..\rs\113.rs:60:42
   |
60 |         Err(err)     => println!("Error: {err}"),
   |                                          ^^^^^ `ReadUsernameError` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `ReadUsernameError`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `ReadUsernameError` doesn't implement `std::fmt::Display`
  --> ..\rs\113.rs:70:42
   |
70 |         Err(err)     => println!("Error: {err}"),
   |                                          ^^^^^ `ReadUsernameError` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `ReadUsernameError`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 10 previous errors

Some errors have detailed explanations: E0277, E0432.
For more information about an error, try `rustc --explain E0277`.
*/