// 116.rs
// https://google.github.io/comprehensive-rust/zh-CN/testing/unit-tests.html

// See: example_116\src\main.rs
// I don't know how to debug it directly. So I converted it to a new project and debugged it.

fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(idx) => &text[..idx],
        None => &text,
    }
}

#[test]
fn test_empty() {
    assert_eq!(first_word(""), "");
}

#[test]
fn test_single_word() {
    assert_eq!(first_word("Hello"), "Hello");
}

#[test]
fn test_multiple_words() {
    assert_eq!(first_word("Hello World"), "Hello");
}

/* result:

*/
