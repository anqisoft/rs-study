// 100.rs
// https://google.github.io/comprehensive-rust/zh-CN/traits/read-write.html

use std::io::{Result, Write};

fn log<W: Write>(writer: &mut W, msg: &str) -> Result<()> {
    writer.write_all(msg.as_bytes())?;
    writer.write_all("\n".as_bytes())
}

fn main() -> Result<()> {
    let mut buffer = Vec::new();
    log(&mut buffer, "Hello")?;
    log(&mut buffer, "World")?;
    println!("Logged: {:?}", buffer);
    Ok(())
}

/* result:
Logged: [72, 101, 108, 108, 111, 10, 87, 111, 114, 108, 100, 10]
*/
