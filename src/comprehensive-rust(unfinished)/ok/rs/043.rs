// 043.rs
// https://google.github.io/comprehensive-rust/zh-CN/control-flow/match-expressions.html

#[rustfmt::skip]
fn main() {
    match std::env::args().next().as_deref() {
        Some("cat") => println!("Will do cat things"),
        Some("ls")  => println!("Will ls some files"),
        Some("mv")  => println!("Let's move some files"),
        Some("rm")  => println!("Uh, dangerous!"),
        None        => println!("Hmm, no program name?"),
        _           => println!("Unknown program name!"),
    }
}

/* result:
Unknown program name!
*/
