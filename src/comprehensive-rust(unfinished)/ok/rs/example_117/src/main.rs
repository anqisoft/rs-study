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

*/
