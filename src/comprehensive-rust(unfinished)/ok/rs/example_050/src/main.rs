// 050.rs
// https://google.github.io/comprehensive-rust/zh-CN/exercises/day-1/luhn.html

// TODO: remove this when you're done with your implementation.
// #![allow(unused_variables, dead_code)]

use regex::Regex;
pub fn luhn(cc_number: &str) -> bool {
    // let re = Regex::new(r"(\d+)").unwrap();

    // let new_number: &str = "";
    // for cap in re.captures_iter(cc_number) {
    //     new_number += cap;
    // }
    let re: Regex = Regex::new(r"([^0-9 ])").unwrap();
    if re.replace_all(cc_number, "").len() > 0 {
        return false;
    }

    let re = Regex::new(r"([^0-9])").unwrap();
    let new_number = re.replace_all(cc_number, "");
    println!("new_number: {new_number}");

    // help: convert the identifier to snake case: `digit_count`    
    // let DIGIT_COUNT = new_number.len();
    let digit_count = new_number.len();
    if digit_count < 2 {
        return false;
    }

    let is_even = digit_count % 2 == 0;

    let mut sum: u32 = 0;
    // https://cloud.tencent.com/developer/ask/sof/93528
    for i in (0..digit_count).rev() {
        let c = &new_number[i..(i+1)];
        println!("{i} => c: {c}");
        let n = c.parse::<u32>().unwrap();
        println!("n: {n}, is_even: {is_even}, i % 2: {}", i % 2);
        if (is_even && i % 2 == 0) || (!is_even && i % 2 == 1) {
            sum += n;
            println!("Add directly! sum: {sum}");
        } else {
            let twice: u32 = n * 2;
            if twice > 9 {
                sum += 1 + (twice % 10);
            } else {
                sum += twice;
            }
            println!("Add indirectly! sum: {sum}, twice: {twice}");
        }
    }
    println!("Result: sum is {sum}, sum % 10 is {}", sum % 10);


    return sum % 10 == 0;
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
fn main() {
    // cannot find function `test_non_digit_cc_number` in this scope
    // test_non_digit_cc_number();
    // cannot find function `test_empty_cc_number` in this scope
    // test_empty_cc_number();
    // cannot find function `test_single_digit_cc_number` in this scope
    // test_single_digit_cc_number();
    // cannot find function `test_two_digit_cc_number` in this scope
    // test_two_digit_cc_number();
    // cannot find function `test_valid_cc_number` in this scope
    // test_valid_cc_number();
    // cannot find function `test_invalid_cc_number` in this scope
    // test_invalid_cc_number();
    
    println!("\nIn test_non_digit_cc_number()");
    assert!(!luhn("foo"));
    assert!(!luhn("foo 0 0"));

    println!("\nIn test_empty_cc_number()");
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));

    println!("\nIn test_single_digit_cc_number()");
    assert!(!luhn("0"));

    println!("\nIn test_two_digit_cc_number()");
    assert!(luhn(" 0 0 "));

    println!("\nIn test_valid_cc_number()");
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));

    println!("\nIn test_invalid_cc_number()");
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}
/* result:

*/
