// error\013.rs
// https://google.github.io/comprehensive-rust/zh-CN/ownership/moves-function-calls.html

fn say_hello(name: String) {
    println!("Hello {name}")
}

fn main() {
    let name = String::from("Alice");
    say_hello(name);
    say_hello(name);
}

/* result:
error[E0382]: use of moved value: `name`
  --> ..\rs\013.rs:11:15
   |
9  |     let name = String::from("Alice");
   |         ---- move occurs because `name` has type `String`, which does not implement the `Copy` trait
10 |     say_hello(name);
   |               ---- value moved here
11 |     say_hello(name);
   |               ^^^^ value used here after move
   |
note: consider changing this parameter type in function `say_hello` to borrow instead if owning the value isn't necessary
  --> ..\rs\013.rs:4:20
   |
4  | fn say_hello(name: String) {
   |    ---------       ^^^^^^ this parameter takes ownership of the value
   |    |
   |    in this function
help: consider cloning the value if the performance cost is acceptable
   |
10 |     say_hello(name.clone());
   |                   ++++++++

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
*/
