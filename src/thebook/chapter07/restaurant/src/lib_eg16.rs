use std::fmt::Result;
use std::io::Result as IoResult;

#[allow(dead_code)]
fn function1() -> Result {
    // --snip--
    Ok(())
}

#[allow(dead_code)]
fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}