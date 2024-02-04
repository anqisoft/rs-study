// 012.rs
// https://google.github.io/comprehensive-rust/zh-CN/basic-syntax/rustdoc.html

/// Determine whether the first argument is divisible by the second argument.
///
/// If the second argument is zero, the result is false.
///
/// # Example
/// ```
/// assert!(is_divisible_by(42, 2));
/// ```
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;  // Corner case, early return
    }
    lhs % rhs == 0     // The last expression in a block is the return value
}

fn main() {
    const DIVISOR: u32 = 2;
    for i in 1..=20 {
        println!("is_divisible_by({i}, {DIVISOR}) is {}", is_divisible_by(i, DIVISOR));
    }
}

/* result:
is_divisible_by(1, 2) is false
is_divisible_by(2, 2) is true
is_divisible_by(3, 2) is false
is_divisible_by(4, 2) is true
is_divisible_by(5, 2) is false
is_divisible_by(6, 2) is true
is_divisible_by(7, 2) is false
is_divisible_by(8, 2) is true
is_divisible_by(9, 2) is false
is_divisible_by(10, 2) is true
is_divisible_by(11, 2) is false
is_divisible_by(12, 2) is true
is_divisible_by(13, 2) is false
is_divisible_by(14, 2) is true
is_divisible_by(15, 2) is false
is_divisible_by(16, 2) is true
is_divisible_by(17, 2) is false
is_divisible_by(18, 2) is true
is_divisible_by(19, 2) is false
is_divisible_by(20, 2) is true
*/

/* cargo_test
(cargo test ..\rs\012.ts )  & pause
    Finished test [unoptimized + debuginfo] target(s) in 0.03s
     Running unittests src\main.rs (P:\anqi\Desktop\tech\rust\projects\rust-study\target\debug\deps\rust_study-2a342569ea370209.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
*/