// 098.rs
// https://google.github.io/comprehensive-rust/zh-CN/traits/from-into.html

fn test1() {
    println!("In test1()");
    let s = String::from("hello");
    let addr = std::net::Ipv4Addr::from([127, 0, 0, 1]);
    let one = i16::from(true);
    let bigger = i32::from(123i16);
    println!("{s}, {addr}, {one}, {bigger}");
}

fn test2() {
    println!("\nIn test2()");
    let s: String = "hello".into();
    let addr: std::net::Ipv4Addr = [127, 0, 0, 1].into();
    let one: i16 = true.into();
    let bigger: i32 = 123i16.into();
    println!("{s}, {addr}, {one}, {bigger}");
}

fn main() {
    test1();
    test2();
}

/* result:
In test1()
hello, 127.0.0.1, 1, 123

In test2()
hello, 127.0.0.1, 1, 123
*/
