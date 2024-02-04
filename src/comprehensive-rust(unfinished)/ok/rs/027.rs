// 027.rs
// https://google.github.io/comprehensive-rust/zh-CN/basic-syntax/variables.html

fn main() {
    test1();
    test2();
    test3();
}

fn test1() {
    println!("In test1()");
    let x: i32 = 10;
    println!("x: {x}");
    // x = 20; // cannot assign twice to immutable variable
    // println!("x: {x}");
}

fn test2() {
    println!("\nIn test2()");
    let x: i32 = 10;
    println!("x: {x}");

    let x: i32;
    x = 20;
    println!("x: {x}");
}

fn test3() {
    println!("\nIn test3()");
    let mut x: i32 = 10;
    println!("x: {x}");

    x = 30;
    println!("x: {x}");
}

/* result:
In test1()
x: 10

In test2()
x: 10
x: 20

In test3()
x: 10
x: 30
*/
