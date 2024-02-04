// 054.rs
// https://google.github.io/comprehensive-rust/zh-CN/ownership/move-semantics.html

fn main() {
    test1();
    test2();
}

fn test1() {
    println!("In test1()");

    let s1: String = String::from("Hello!");
    let s2: String = s1;
    println!("s2: {s2}");
    // println!("s1: {s1}");
}

fn test2() {
    println!();
    println!("In test2()");

    let s1: String = String::from("Hello!");
    let s2: String = s1.clone();
    println!("s2: {s2}");
    println!("s1: {s1}");
}

/* result:
In test1()
s2: Hello!

In test2()
s2: Hello!
s1: Hello!
*/
