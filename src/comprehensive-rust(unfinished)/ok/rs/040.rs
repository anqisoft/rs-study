// 040.rs
// https://google.github.io/comprehensive-rust/zh-CN/control-flow/if-let-expressions.html

fn main() {
    let arg = std::env::args().next();
    if let Some(value) = arg {
        println!("Program name: {value}");
    } else {
        println!("Missing name?");
    }
}

/* result:
Program name: 040
*/

/* result in page:
Program name: target/debug/playground
*/
