// 010.rs
// https://google.github.io/comprehensive-rust/zh-CN/basic-syntax/string-slices.html

fn main() {
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[6..];
    println!("s3: {s3}");

    test1();
    test2();
}

fn test1() {
    println!("\nIn test1()");
    let mut s1: &str = "World";
    println!("s1: {s1}");

    s1 = "Rust";
    println!("s1: {s1}");

    // s1.push_str(" test"); // method not found in `&str`
}

fn test2() {
    println!("\nIn test2()");
    let mut s1: String = String::from("Hello ");
    println!("s1: {s1}");

    s1 = String::from("Rust ");
    println!("s1: {s1}");
}

/* result:
s1: World
s2: Hello
s2: Hello World
s3: World
*/
