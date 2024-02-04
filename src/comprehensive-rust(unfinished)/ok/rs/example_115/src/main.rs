// 115.rs
// https://google.github.io/comprehensive-rust/zh-CN/error-handling/error-contexts.html

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

// fn main() {
//     //fs::write("config.dat", "").unwrap();
//     match read_username("config.dat") {
//         Ok(username) => println!("Username: {username}"),
//         Err(err)     => println!("Error: {err}"),
//     }
// }

fn main() {
    test1();
    test2();
    test3();
}

const FILE_NAME: &str = "config.dat";
fn test1() {
    println!("In test1()");
    use std::path::Path;
    if Path::new(FILE_NAME).exists() {
        fs::remove_file(FILE_NAME).unwrap();
    }

    //fs::write(FILE_NAME, "").unwrap();
    match read_username(FILE_NAME) {
        Ok(username) => println!("Username: {username}"),
        Err(err)     => println!("Error: {err}"),
    }
}

fn test2() {
    println!("\nIn test2()");
    fs::write(FILE_NAME, "").unwrap();

    match read_username(FILE_NAME) {
        Ok(username) => println!("Username: {username}"),
        Err(err)     => println!("Error: {err}"),
    }
}

fn test3() {
    println!("\nIn test3()");
    fs::write(FILE_NAME, "alice").unwrap();

    match read_username(FILE_NAME) {
        Ok(username) => println!("Username: {username}"),
        Err(err)     => println!("Error: {err}"),
    }
}

/* result:
In test1()
Error: Failed to open config.dat

In test2()
Error: Found no username in config.dat

In test3()
Username: alice
*/

/*
warning: unused import: `io`
 --> src\main.rs:4:15
  |
4 | use std::{fs, io};
  |               ^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `example_115` (bin "example_115") generated 1 warning (run `cargo fix --bin "example_115"` to apply 1 suggestion)
    Finished dev [unoptimized + debuginfo] target(s) in 1.76s
     Running `target\debug\example_115.exe`
*/