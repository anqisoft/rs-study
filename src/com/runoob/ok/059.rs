// 059.rs
// https://www.runoob.com/rust/rust-file-io.html

use std::io::stdin;

fn main() {
    let mut str_buf = String::new();

    stdin().read_line(&mut str_buf)
        .expect("Failed to read line.");

    println!("Your input line is \n{}", str_buf);
}

/* result:
Your input line is
11111
*/
