// 065.rs
// https://www.runoob.com/rust/rust-file-io.html

use std::io::prelude::*;
use std::fs::OpenOptions;

fn main() -> std::io::Result<()> {

    let mut file = OpenOptions::new()
        .append(true).open("D:\\text.txt")?;

    file.write(b" APPEND WORD")?;

    Ok(())
}

/* result: (empty)

*/
