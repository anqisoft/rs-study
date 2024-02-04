// 036.rs
// https://google.github.io/comprehensive-rust/zh-CN/enums/sizes.html

use std::any::type_name;
use std::mem::{align_of, size_of};

fn dbg_size<T>() {
    println!("{}: size {} bytes, align: {} bytes",
        type_name::<T>(), size_of::<T>(), align_of::<T>());
}

enum Foo {
    A,
    B,
}

fn main() {
    dbg_size::<Foo>();
}

/* result:
036::Foo: size 1 bytes, align: 1 bytes
*/

/* warnings:
warning: variants `A` and `B` are never constructed
  --> ..\rs\036.rs:13:5
   |
12 | enum Foo {
   |      --- variants in this enum
13 |     A,
   |     ^
14 |     B,
   |     ^
   |
   = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted
*/