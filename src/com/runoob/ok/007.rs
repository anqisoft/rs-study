// 007.rs
// https://www.runoob.com/rust/rust-function.html

fn main() {
    println!("Hello, world!");
    another_function();
    another_function2(5, 6);
}

fn another_function() {
    println!("Hello, runoob!");
}

fn another_function2(x: i32, y: i32) {
    println!("x is {}", x);
    println!("y is {}", y);
}

/* result:
Hello, world!
Hello, runoob!
x is 5
y is 6
*/
