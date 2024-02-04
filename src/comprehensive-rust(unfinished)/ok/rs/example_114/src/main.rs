// 114.rs
// https://google.github.io/comprehensive-rust/zh-CN/error-handling/deriving-error-enums.html

// See: example_114\src\main.rs
// I don't know how to debug it directly. So I converted it to a new project and debugged it.

use std::fs;
use std::io::Read;
use thiserror::Error;
use std::error::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("Found no username in {0}")]
struct EmptyUsernameError(String);

fn read_username(path: &str) -> Result<String, Box<dyn Error>> {
    let mut username = String::new();
    fs::File::open(path)?.read_to_string(&mut username)?;
    if username.is_empty() {
        return Err(EmptyUsernameError(String::from(path)).into());
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