// 119.rs
// https://google.github.io/comprehensive-rust/zh-CN/testing/integration-tests.html

// See: example_119\src\main.rs
// I don't know how to debug it directly. So I converted it to a new project and debugged it.

// If you want to test your library as a client, use integration tests.
// Create a .rs file under tests/:

use my_library::init;

#[test]
fn test_init() {
    assert!(init().is_ok());
}