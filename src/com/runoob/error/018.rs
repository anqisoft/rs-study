// error/018.rs
// https://www.runoob.com/rust/rust-struct.html
struct Dog {
    name: String,
    age: i8
}

fn main() {
    let mydog = Dog {
        name:String::from("wangcai"),
        age:3,
    };
    let str = mydog.name;
    println!("str={}", str);
    println!("mydog: name={},age={}", mydog.name, mydog.age);
}

/* result:
error[E0382]: borrow of moved value: `mydog.name`
  --> ..\018.rs:15:39
   |
13 |     let str = mydog.name;
   |               ---------- value moved here
14 |     println!("str={}", str);
15 |     println!("mydog: name={},age={}", mydog.name, mydog.age);
   |                                       ^^^^^^^^^^ value borrowed here after move
   |
   = note: move occurs because `mydog.name` has type `String`, which does not implement the `Copy` trait
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
*/
