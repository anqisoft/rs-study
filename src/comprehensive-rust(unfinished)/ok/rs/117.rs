// 117.rs
// https://google.github.io/comprehensive-rust/zh-CN/testing/test-modules.html

// See: example_117\src\main.rs
// I don't know how to debug it directly. So I converted it to a new project and debugged it.

fn helper(a: &str, b: &str) -> String {
    format!("{a} {b}")
}

pub fn main() {
    println!("{}", helper("Hello", "World"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helper() {
        assert_eq!(helper("foo", "bar"), "foo bar");
    }
}

/* result:
Hello World
*/

/* test
running 1 test
test tests::test_helper ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
*/