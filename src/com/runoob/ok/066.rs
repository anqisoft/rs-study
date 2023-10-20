// 066.rs
// https://www.runoob.com/rust/rust-file-io.html

use std::io::prelude::*;
use std::fs::OpenOptions;

fn main() -> std::io::Result<()> {

    let mut file = OpenOptions::new()
        .read(true).write(true).open("D:\\text.txt")?;

    file.write(b"COVER")?;

    Ok(())
}

/* result: (empty)

*/
