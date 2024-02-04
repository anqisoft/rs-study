use std::fmt;
use std::io;

#[allow(dead_code)]
fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

#[allow(dead_code)]
fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}
