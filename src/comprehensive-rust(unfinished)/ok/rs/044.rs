// 044.rs
// https://google.github.io/comprehensive-rust/zh-CN/pattern-matching.html

#[rustfmt::skip]
fn main() {
    let input = 'x';

    match input {
        'q'                   => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9'             => println!("Number input"),
        _                     => println!("Something else"),
    }
}

/* result:
Something else
*/
