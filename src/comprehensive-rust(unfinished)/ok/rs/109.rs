// 109.rs
// https://google.github.io/comprehensive-rust/zh-CN/error-handling/panic-unwind.html

use std::panic;

fn main() {
    let result = panic::catch_unwind(|| {
        println!("hello!");
    });
    assert!(result.is_ok());
    
    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });
    assert!(result.is_err());
}

/* result:
hello!
*/

/*
thread 'main' panicked at ..\rs\109.rs:13:9:
oh no!
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/