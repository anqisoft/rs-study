// error\016.rs
// https://google.github.io/comprehensive-rust/zh-CN/ownership/shared-unique-borrows.html

fn main() {
    let mut a: i32 = 10;
    let b: &i32 = &a;

    {
        let c: &mut i32 = &mut a;
        *c = 20;
    }

    println!("a: {a}");
    println!("b: {b}");
}

/* result:
error[E0502]: cannot borrow `a` as mutable because it is also borrowed as immutable
  --> ..\rs\016.rs:9:27
   |
6  |     let b: &i32 = &a;
   |                   -- immutable borrow occurs here
...
9  |         let c: &mut i32 = &mut a;
   |                           ^^^^^^ mutable borrow occurs here
...
14 |     println!("b: {b}");
   |                  --- immutable borrow later used here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0502`.
*/
