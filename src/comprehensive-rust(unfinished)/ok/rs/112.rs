// 112.rs
// https://google.github.io/comprehensive-rust/zh-CN/error-handling/converting-error-types-example.html

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, File};
use std::io::{self, Read};

#[derive(Debug)]
enum ReadUsernameError {
    IoError(io::Error),
    EmptyUsername(String),
}

impl Error for ReadUsernameError {}

impl Display for ReadUsernameError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO error: {e}"),
            Self::EmptyUsername(filename) => write!(f, "Found no username in {filename}"),
        }
    }
}

impl From<io::Error> for ReadUsernameError {
    fn from(err: io::Error) -> ReadUsernameError {
        ReadUsernameError::IoError(err)
    }
}

fn read_username(path: &str) -> Result<String, ReadUsernameError> {
    let mut username = String::with_capacity(100);
    File::open(path)?.read_to_string(&mut username)?;
    if username.is_empty() {
        return Err(ReadUsernameError::EmptyUsername(String::from(path)));
    }
    Ok(username)
}

// fn main() {
//     //fs::write("config.dat", "").unwrap();
//     let username = read_username("config.dat");
//     println!("username or error: {username:?}");
// }

fn main() {
    test1();
    test2();
}

const FILE_NAME: &str = "config.dat";
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
username or error: Err(IoError(Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }))

In test2()
username or error: Ok("alice")
*/
