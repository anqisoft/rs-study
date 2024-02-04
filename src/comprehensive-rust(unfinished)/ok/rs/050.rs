// 050.rs
// https://google.github.io/comprehensive-rust/zh-CN/exercises/day-1/luhn.html

// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    // let re = Regex::new(r"(\d+)").unwrap();

    // let new_number: &str = "";
    // for cap in re.captures_iter(cc_number) {
    //     new_number += cap;
    // }
    let re = Regex::new(r"([^0-9])").unwrap();

    let new_number = re.replace_all(cc_number, "");
    println!("new_number: {new_number}");
    
    new_number.length > 2
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
    assert!(!luhn("foo 0 0"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

#[allow(dead_code)]
fn main() {}
/* result:

*/
