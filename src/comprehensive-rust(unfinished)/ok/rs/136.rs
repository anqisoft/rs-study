// 136.rs
// https://google.github.io/comprehensive-rust/zh-CN/async/futures.html
// https://doc.rust-lang.org/std/task/enum.Poll.html

use std::pin::Pin;
use std::task::Context;

pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}

fn main() {
    // pub fn map<U, F>(self, f: F) -> Poll<U> where F: FnOnce(T) -> U,
    let poll_some_string = Poll::Ready(String::from("Hello, World!"));
    // `Poll::map` takes self *by value*, consuming `poll_some_string`
    let poll_some_len = poll_some_string.map(|s| s.len());

    assert_eq!(poll_some_len, Poll::Ready(13));

    // pub const fn is_ready(&self) -> bool
    let x: Poll<u32> = Poll::Ready(2);
    assert_eq!(x.is_ready(), true);

    let x: Poll<u32> = Poll::Pending;
    assert_eq!(x.is_ready(), false);

    // pub const fn is_pending(&self) -> bool
    let x: Poll<u32> = Poll::Ready(2);
    assert_eq!(x.is_pending(), false);

    let x: Poll<u32> = Poll::Pending;
    assert_eq!(x.is_pending(), true);

    // pub fn map_ok<U, F>(self, f: F) -> Poll<Result<U, E>> where F: FnOnce(T) -> U,
    let res: Poll<Result<u8, _>> = Poll::Ready("12".parse());
    let squared = res.map_ok(|n| n * n);
    assert_eq!(squared, Poll::Ready(Ok(144)));

    // pub fn map_err<U, F>(self, f: F) -> Poll<Result<T, U>> where F: FnOnce(E) -> U,
    let res: Poll<Result<u8, _>> = Poll::Ready("oops".parse());
    let res = res.map_err(|_| 0_u8);
    assert_eq!(res, Poll::Ready(Err(0)));
}

/* result:
error[E0599]: `Poll<String>` is not an iterator
  --> ..\rs\136.rs:22:42
   |
13 | pub enum Poll<T> {
   | ----------------
   | |
   | method `map` not found for this enum
   | doesn't satisfy `Poll<String>: Iterator`
...
22 |     let poll_some_len = poll_some_string.map(|s| s.len());
   |                                          ^^^ `Poll<String>` is not an iterator
   |
   = note: the following trait bounds were not satisfied:
           `Poll<String>: Iterator`
           which is required by `&mut Poll<String>: Iterator`
note: the trait `Iterator` must be implemented
  --> D:\anqi\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\traits\iterator.rs:74:1
   |
74 | pub trait Iterator {
   | ^^^^^^^^^^^^^^^^^^
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `map`, perhaps you need to implement it:
           candidate #1: `Iterator`

error[E0599]: no method named `is_ready` found for enum `Poll` in the current scope
  --> ..\rs\136.rs:28:18
   |
13 | pub enum Poll<T> {
   | ---------------- method `is_ready` not found for this enum
...
28 |     assert_eq!(x.is_ready(), true);
   |                  ^^^^^^^^ method not found in `Poll<u32>`

error[E0599]: no method named `is_ready` found for enum `Poll` in the current scope
  --> ..\rs\136.rs:31:18
   |
13 | pub enum Poll<T> {
   | ---------------- method `is_ready` not found for this enum
...
31 |     assert_eq!(x.is_ready(), false);
   |                  ^^^^^^^^ method not found in `Poll<u32>`

error[E0599]: no method named `is_pending` found for enum `Poll` in the current scope
  --> ..\rs\136.rs:35:18
   |
13 | pub enum Poll<T> {
   | ---------------- method `is_pending` not found for this enum
...
35 |     assert_eq!(x.is_pending(), false);
   |                  ^^^^^^^^^^ method not found in `Poll<u32>`

error[E0599]: no method named `is_pending` found for enum `Poll` in the current scope
  --> ..\rs\136.rs:38:18
   |
13 | pub enum Poll<T> {
   | ---------------- method `is_pending` not found for this enum
...
38 |     assert_eq!(x.is_pending(), true);
   |                  ^^^^^^^^^^ method not found in `Poll<u32>`

error[E0599]: no method named `map_ok` found for enum `Poll` in the current scope
  --> ..\rs\136.rs:42:23
   |
13 | pub enum Poll<T> {
   | ---------------- method `map_ok` not found for this enum
...
42 |     let squared = res.map_ok(|n| n * n);
   |                       ^^^^^^ method not found in `Poll<Result<u8, ParseIntError>>`

error[E0599]: no method named `map_err` found for enum `Poll` in the current scope
   --> ..\rs\136.rs:47:19
    |
13  | pub enum Poll<T> {
    | ---------------- method `map_err` not found for this enum
...
47  |     let res = res.map_err(|_| 0_u8);
    |                   ^^^^^^^ method not found in `Poll<Result<u8, ParseIntError>>`
    |
note: the method `map_err` exists on the type `Result<u8, ParseIntError>`
   --> D:\anqi\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\result.rs:826:5
    |
826 |     pub fn map_err<F, O: FnOnce(E) -> F>(self, op: O) -> Result<T, F> {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0599`.
*/
