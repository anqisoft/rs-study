// 073.rs
// https://google.github.io/comprehensive-rust/zh-CN/std/string.html

fn main() {
    let mut s1 = String::new();
    s1.push_str("Hello");
    println!("s1: len = {}, capacity = {}", s1.len(), s1.capacity());

    let mut s2 = String::with_capacity(s1.len() + 1);
    s2.push_str(&s1);
    s2.push('!');
    println!("s2: len = {}, capacity = {}", s2.len(), s2.capacity());

    let s3 = String::from("🇨🇭");
    println!("s3: len = {}, number of chars = {}", s3.len(),
             s3.chars().count());
}

/* result:
s1: len = 5, capacity = 8
s2: len = 6, capacity = 6
s3: len = 8, number of chars = 2
*/
