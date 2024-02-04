// 102.rs
// https://google.github.io/comprehensive-rust/zh-CN/traits/default.html

#[derive(Debug, Default)]
struct Derived {
    x: u32,
    y: String,
    z: Implemented,
}

#[derive(Debug)]
struct Implemented(String);

impl Default for Implemented {
    fn default() -> Self {
        Self("John Smith".into())
    }
}

fn main() {
    let default_struct = Derived::default();
    println!("{default_struct:#?}");

    let almost_default_struct = Derived {
        y: "Y is set!".into(),
        ..Derived::default()
    };
    println!("{almost_default_struct:#?}");

    let nothing: Option<Derived> = None;
    println!("{:#?}", nothing.unwrap_or_default());
}

/* result:
Derived {
    x: 0,
    y: "",
    z: Implemented(
        "John Smith",
    ),
}
Derived {
    x: 0,
    y: "Y is set!",
    z: Implemented(
        "John Smith",
    ),
}
Derived {
    x: 0,
    y: "",
    z: Implemented(
        "John Smith",
    ),
}
*/
