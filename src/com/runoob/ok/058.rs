// 058.rs
// https://www.runoob.com/rust/rust-file-io.html

fn main() {
    let args = std::env::args();
    for arg in args {
        println!("{}", arg);
    }
}

/* result:
058
*/
