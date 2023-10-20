// 020.rs
// https://www.runoob.com/rust/rust-ownership.html

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

/* result:
The length of 'hello' is 5.
*/
