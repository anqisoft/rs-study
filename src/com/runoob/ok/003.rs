// 003.rs
// https://www.runoob.com/rust/rust-basic-syntax.html

// fn main() {
//     let mut a = 123;
//     a = 456;
//     println!("Hello, Rustaceans! Say by 003.rs.");
// }

/* result:
warning: variable `a` is assigned to, but never used
 --> ..\003.rs:5:13
  |
5 |     let mut a = 123;
  |             ^
  |
  = note: consider using `_a` instead
  = note: `#[warn(unused_variables)]` on by default

warning: value assigned to `a` is never read
 --> ..\003.rs:6:5
  |
6 |     a = 456;
  |     ^
  |
  = help: maybe it is overwritten before being read?
  = note: `#[warn(unused_assignments)]` on by default

warning: 2 warnings emitted

Hello, Rustaceans! Say by 003.rs.
*/

// fn main() {
//     let mut a = 123;
//     a = 456;
//     println!("Hello, Rustaceans!  Say by 003.rs.");
//     println!("a is {}", a);
// }

/* result:
warning: value assigned to `a` is never read
  --> ..\003.rs:35:13
   |
35 |     let mut a = 123;
   |             ^
   |
   = help: maybe it is overwritten before being read?
   = note: `#[warn(unused_assignments)]` on by default

warning: 1 warning emitted

Hello, Rustaceans!  Say by 003.rs.
a is 456
*/

// fn main() {
//     let a = 123;
//     let a = 456;
//     println!("a is {}", a);
// }

/* result:
warning: unused variable: `a`
  --> ..\003.rs:58:9
   |
58 |     let a = 123;
   |         ^ help: if this is intentional, prefix it with an underscore: `_a`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: 1 warning emitted

a is 456
*/

// fn main() {
//     const a: i32 = 123;
//     let a = 456;
//     println!("a is {}", a);
//   }
  
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
  
  fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
} 

/* result:
// The value of x is: 12
  */