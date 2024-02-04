// 079.rs
// https://google.github.io/comprehensive-rust/zh-CN/std/rc.html

use std::rc::Rc;

fn main() {
    let mut a = Rc::new(10);
    let mut b = Rc::clone(&a);

    println!("a: {a}");
    println!("b: {b}");
}

/* result:
a: 10
b: 10
*/

/*
warning: variable does not need to be mutable
 --> ..\rs\079.rs:7:9
  |
7 |     let mut a = Rc::new(10);
  |         ----^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default

warning: variable does not need to be mutable
 --> ..\rs\079.rs:8:9
  |
8 |     let mut b = Rc::clone(&a);
  |         ----^
  |         |
  |         help: remove this `mut`

warning: 2 warnings emitted
*/