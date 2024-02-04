// 111.rs
// https://google.github.io/comprehensive-rust/zh-CN/error-handling/try-operator.html

use std::{fs, io};
use std::io::Read;

const FILE_NAME: &str = "config.dat";

fn read_username(path: &str) -> Result<String, io::Error> {
    let username_file_result = fs::File::open(path);
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(err) => Err(err),
    }
}

fn main() {
    test1();
    test2();
}

fn test1() {
    println!("In test1()");
    use std::path::Path;
    if Path::new(FILE_NAME).exists() {
        fs::remove_file(FILE_NAME).unwrap();
    }

    let username = read_username(FILE_NAME);
    println!("username or error: {username:?}");
}

fn test2() {
    println!("\nIn test2()");
    fs::write(FILE_NAME, "alice").unwrap();

    let username = read_username(FILE_NAME);
    println!("username or error: {username:?}");
}

/* result:
In test1()
username or error: Err(Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." })

In test2()
username or error: Ok("alice")
*/
