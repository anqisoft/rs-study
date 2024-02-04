// 062.rs
// https://google.github.io/comprehensive-rust/zh-CN/ownership/lifetimes-data-structures.html

#[derive(Debug)]
struct Highlight<'doc>(&'doc str);

fn erase(text: String) {
    println!("Bye {text}!");
}

fn main() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);

    //  move out of `text` occurs here
    // erase(text);
    
    println!("{fox:?}");
    println!("{dog:?}");
}

/* result:
Highlight("quick brown fox")
Highlight("lazy dog")
*/

/* warning
warning: function `erase` is never used
 --> ..\rs\062.rs:7:4
  |
7 | fn erase(text: String) {
  |    ^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted
*/