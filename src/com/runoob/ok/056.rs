// 056.rs
// https://www.runoob.com/rust/rust-lifetime.html

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    println!("longest_with_an_announcement(\"{}\", \"{}\", {}) is {}", "abcde", "fgh", 200, longest_with_an_announcement("abcde", "fgh", 200));
}

/* result:
Announcement! 200
longest_with_an_announcement("abcde", "fgh", 200) is abcde
*/
