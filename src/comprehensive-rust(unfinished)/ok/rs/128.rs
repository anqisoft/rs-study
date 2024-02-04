// 128.rs
// https://google.github.io/comprehensive-rust/zh-CN/concurrency/scoped-threads.html

use std::thread;

fn test1() {
    println!("In test1()");
    fn foo() {
        let s = String::from("Hello");
        /*
error[E0373]: closure may outlive the current function, but it borrows `s`, which is owned by the current function
  --> ..\rs\128.rs:10:23
   |
10 |         thread::spawn(|| {
   |                       ^^ may outlive borrowed value `s`
11 |             println!("Length: {}", s.len());
   |                                    - `s` is borrowed here
   |
note: function requires argument type to outlive `'static`
  --> ..\rs\128.rs:10:9
   |
10 | /         thread::spawn(|| {
11 | |             println!("Length: {}", s.len());
12 | |         });
   | |__________^
help: to force the closure to take ownership of `s` (and any other referenced variables), use the `move` keyword
   |
10 |         thread::spawn(move || {
   |                       ++++

error: aborting due to previous error
*/
        // thread::spawn(|| {
        //     println!("Length: {}", s.len());
        // });

        thread::spawn(move || {
            println!("test1(), Length: {}", s.len());
        });
    }

    foo();
}

fn test2() {
    println!("\nIn test2()");

    let s = String::from("Hello");

    thread::scope(|scope| {
        scope.spawn(|| {
            println!("test2(), Length: {}", s.len());
        });
    });
}

fn main() {
    test1();
    test2();
}

/* result:
In test1()

In test2()
test1(), Length: 5
test2(), Length: 5
*/
