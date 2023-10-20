// 019.rs
// https://www.runoob.com/rust/rust-ownership.html

fn main() {
    let s1 = String::from("hello");
    let s2 = &s1;
    println!("s1 is {}, s2 is {}", s1, s2);
}

/* result:
s1 is hello, s2 is hello
*/
