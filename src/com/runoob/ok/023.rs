// 023.rs
// https://www.runoob.com/rust/rust-ownership.html

fn main() {
    let reference_to_nothing = dangle();
}

// fn dangle() -> &String {
fn dangle() -> &'static String {
    let s = String::from("hello");

    &s
}

/* result:

*/
