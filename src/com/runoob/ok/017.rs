// 017.rs
// https://www.runoob.com/rust/rust-ownership.html

fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");

    println!("before call takes_and_gives_back(), s1 is {}, s2 is {}", s1, s2);
    let s3 = takes_and_gives_back(s2);
    println!("after call takes_and_gives_back(), s1 is {}, s3 is {}", s1, s3);
    println!("after call takes_and_gives_back(), s2 is {}", s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    return some_string;
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

/* result:
before call takes_and_gives_back(), s1 is hello, s2 is hello
after call takes_and_gives_back(), s1 is hello, s3 is hello
*/

/* error if use line11
error[E0382]: borrow of moved value: `s2`
  --> ..\017.rs:11:61
   |
6  |     let s2 = String::from("hello");
   |         -- move occurs because `s2` has type `String`, which does not implement the `Copy` trait
...
9  |     let s3 = takes_and_gives_back(s2);
   |                                   -- value moved here
10 |     println!("after call takes_and_gives_back(), s1 is {}, s3 is {}", s1, s3);
11 |     println!("after call takes_and_gives_back(), s2 is {}", s2);
   |                                                             ^^ value borrowed here after move
   |
note: consider changing this parameter type in function `takes_and_gives_back` to borrow instead if owning the value isn't necessary
  --> ..\017.rs:19:35
   |
19 | fn takes_and_gives_back(a_string: String) -> String {
   |    --------------------           ^^^^^^ this parameter takes ownership of the value
   |    |
   |    in this function
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
   |
9  |     let s3 = takes_and_gives_back(s2.clone());
   |                                     ++++++++

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
*/