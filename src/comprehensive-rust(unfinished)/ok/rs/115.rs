// 115.rs
// https://google.github.io/comprehensive-rust/zh-CN/error-handling/error-contexts.html

// See: example_115\src\main.rs
// I don't know how to debug it directly. So I converted it to a new project and debugged it.

use std::{fs, io};
use std::io::Read;
use anyhow::{Context, Result, bail};

fn read_username(path: &str) -> Result<String> {
    let mut username = String::with_capacity(100);
    fs::File::open(path)
        .with_context(|| format!("Failed to open {path}"))?
        .read_to_string(&mut username)
        .context("Failed to read")?;
    if username.is_empty() {
        bail!("Found no username in {path}");
    }
    Ok(username)
}

fn main() {
    //fs::write("config.dat", "").unwrap();
    match read_username("config.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err)     => println!("Error: {err:?}"),
    }
}

/*
error[E0432]: unresolved import `anyhow`
 --> ..\rs\115.rs:6:5
  |
6 | use anyhow::{Context, Result, bail};
  |     ^^^^^^ maybe a missing crate `anyhow`?
  |
  = help: consider adding `extern crate anyhow` to use the `anyhow` crate

error: cannot determine resolution for the macro `bail`
  --> ..\rs\115.rs:15:9
   |
15 |         bail!("Found no username in {path}");
   |         ^^^^
   |
   = note: import resolution is stuck, try simplifying macro imports

warning: unused import: `io`
 --> ..\rs\115.rs:4:15
  |
4 | use std::{fs, io};
  |               ^^
  |
  = note: `#[warn(unused_imports)]` on by default

error[E0599]: no method named `with_context` found for enum `Result` in the current scope
  --> ..\rs\115.rs:11:10
   |
10 | /     fs::File::open(path)
11 | |         .with_context(|| format!("Failed to open {path}"))?
   | |         -^^^^^^^^^^^^ method not found in `Result<File, Error>`
   | |_________|
   |

warning: unused import: `std::io::Read`
 --> ..\rs\115.rs:5:5
  |
5 | use std::io::Read;
  |     ^^^^^^^^^^^^^

error: aborting due to 3 previous errors; 2 warnings emitted

Some errors have detailed explanations: E0432, E0599.
For more information about an error, try `rustc --explain E0432`.
*/
