// 118.rs
// https://google.github.io/comprehensive-rust/zh-CN/testing/doc-tests.html

// See: example_118\src\main.rs
// I don't know how to debug it directly. So I converted it to a new project and debugged it.

/// Shortens a string to the given length.
///
/// ```
/// # use playground::shorten_string;
/// assert_eq!(shorten_string("Hello World", 5), "Hello");
/// assert_eq!(shorten_string("Hello World", 20), "Hello World");
/// ```
pub fn shorten_string(s: &str, length: usize) -> &str {
    &s[..std::cmp::min(length, s.len())]
}

/* result:

*/
