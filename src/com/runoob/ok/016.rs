// 016.rs
// https://www.runoob.com/rust/rust-ownership.html

fn main() {
    let s = String::from("hello");
    println!("before call takes_ownership(), s is {}", s);
    takes_ownership(s);
    // println!("after call takes_ownership(), s is {}", s);

    let x = 5;
    println!("before call makes_copy(), x is {}", x);
    makes_copy(x);
    println!("after call makes_copy(), x is {}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

/* result:
before call takes_ownership(), s is hello
hello
before call makes_copy(), x is 5
5
after call makes_copy(), x is 5
*/

/* result if use line 8:
error[E0382]: borrow of moved value: `s`
  --> ..\016.rs:8:55
   |
5  |     let s = String::from("hello");
   |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
6  |     println!("before call takes_ownership(), s is {}", s);
7  |     takes_ownership(s);
   |                     - value moved here
8  |     println!("after call takes_ownership(), s is {}", s);
   |                                                       ^ value borrowed here after move
   |
note: consider changing this parameter type in function `takes_ownership` to borrow instead if owning the value isn't necessary
  --> ..\016.rs:16:33
   |
16 | fn takes_ownership(some_string: String) {
   |    ---------------              ^^^^^^ this parameter takes ownership of the value
   |    |
   |    in this function
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
   |
7  |     takes_ownership(s.clone());
   |                      ++++++++

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
 */
