// 055.rs
// https://www.runoob.com/rust/rust-lifetime.html

fn main() {
    struct Str<'a> {
        content: &'a str
    }

    impl<'a> Str<'a> {
        fn get_content(&self) -> &str {
            self.content
        }
    }

    let s = Str {
        content: "string_slice"
    };
    println!("s.content = {}", s.content);
    println!("s.get_content() = {}", s.get_content());
}

/* result:
s.content = string_slice
s.get_content() = string_slice
*/
