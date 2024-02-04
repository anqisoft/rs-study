// error\001.rs
// https://google.github.io/comprehensive-rust/zh-CN/basic-syntax/references.html

fn main() {
    let mut x: i32 = 10;
    println!("x: {x}");

    let ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("x: {x}");

    let ref_x2: &mut i32 = &mut x;
    *ref_x2 = 21;
    println!("x: {x}, ref_x: {ref_x}, ref_x2: {ref_x2}");
}

/* result:
error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable
  --> ..\rs\001.rs:10:18
   |
8  |     let ref_x: &mut i32 = &mut x;
   |                           ------ mutable borrow occurs here
9  |     *ref_x = 20;
10 |     println!("x: {x}");
   |                  ^^^ immutable borrow occurs here
...
14 |     println!("x: {x}, ref_x: {ref_x}, ref_x2: {ref_x2}");
   |                              ------- mutable borrow later used here
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0499]: cannot borrow `x` as mutable more than once at a time
  --> ..\rs\001.rs:12:28
   |
8  |     let ref_x: &mut i32 = &mut x;
   |                           ------ first mutable borrow occurs here
...
12 |     let ref_x2: &mut i32 = &mut x;
   |                            ^^^^^^ second mutable borrow occurs here
13 |     *ref_x2 = 21;
14 |     println!("x: {x}, ref_x: {ref_x}, ref_x2: {ref_x2}");
   |                              ------- first borrow later used here

error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable
  --> ..\rs\001.rs:14:18
   |
8  |     let ref_x: &mut i32 = &mut x;
   |                           ------ mutable borrow occurs here
...
14 |     println!("x: {x}, ref_x: {ref_x}, ref_x2: {ref_x2}");
   |                  ^^^         ------- mutable borrow later used here
   |                  |
   |                  immutable borrow occurs here
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0499, E0502.
For more information about an error, try `rustc --explain E0499`.
*/

