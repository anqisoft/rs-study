// error/005.rs
// https://www.runoob.com/rust/rust-basic-syntax.html


fn main() {
  const a: i32 = 123;
  let a = 456;
  println!("a is {}", a);
}

/* result:
warning: unused variable: `a`
  --> ..\003.rs:79:9
   |
79 |     let a = 123;
   |         ^ help: if this is intentional, prefix it with an underscore: `_a`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: 1 warning emitted

a is 456
*/
