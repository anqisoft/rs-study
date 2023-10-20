// 015.rs
// https://www.runoob.com/rust/rust-ownership.html

fn main() {
    // cannot find value `s` in this scope
    // error: aborting due to previous error
    // println!("s is {}", s);

    {
        let s = "runoob";
    }

    // error[E0425]: cannot find value `s` in this scope
    // help: the binding `s` is available in a different scope in the same function
    // error: aborting due to previous error
    // println!("s is {}", s);

    let x = 5;
    println!("x is {}", x);
    let y = x;
    println!("y is {}", y);
    println!("x is {}", x);

    println!("");
    let s1 = String::from("hello");
    println!("s1 is {}", s1);
    let s2 = s1;
    println!("s2 is {}", s2);
    // println!("s1 is {}", s1);
    /*
        error[E0382]: borrow of moved value: `s1`
          --> ..\015.rs:29:26
           |
        25 |     let s1 = String::from("hello");
           |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
        26 |     println!("s1 is {}", s1);
        27 |     let s2 = s1;
           |              -- value moved here
        28 |     println!("s2 is {}", s2);
        29 |     println!("s1 is {}", s1);
           |                          ^^ value borrowed here after move
           |
           = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
        help: consider cloning the value if the performance cost is acceptable
           |
        27 |     let s2 = s1.clone();
           |                ++++++++

        error: aborting due to previous error; 1 warning emitted

        For more information about this error, try `rustc --explain E0382`.
     */


    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);
}

/* result:
warning: unused variable: `s`
  --> ..\015.rs:10:13
   |
10 |         let s = "runoob";
   |             ^ help: if this is intentional, prefix it with an underscore: `_s`
   |
   = note: `#[warn(unused_variables)]` on by default

x is 5
y is 5
x is 5

s1 is hello
s2 is hello
s3 = hello, s4 = hello
*/
