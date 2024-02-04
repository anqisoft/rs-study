// 003.rs
// https://google.github.io/comprehensive-rust/zh-CN/basic-syntax/scalar-types.html

fn main() {
    println!(r#"<a href="link.html">link</a>"#);
    println!("<a href=\"link.html\">link</a>");
}

/* result:
<a href="link.html">link</a>
<a href="link.html">link</a>
*/
