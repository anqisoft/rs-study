fn for_p197b_to_p199b(read_username_from_file: impl Fn(Result<String, dyn std::error::Error>)) {
    const FILE_NAME: &str = "hello.txt";
    remove_hello_dot_txt_if_exists();
    let result = read_username_from_file();
    println!("result is {result:?}");

    remove_hello_dot_txt_if_exists();
    File::create(FILE_NAME).unwrap_or_else(|error| {
        panic!("Problem creating the file: {:?}", error);
    });
    let result = read_username_from_file();
    println!("result is {result:?}");

    remove_hello_dot_txt_if_exists();
}

fn p197b() {
    println!("\nIn p197b(), eg9-6");

    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    for_p197b_to_p199b(read_username_from_file);
}

fn p198a() {
    println!("\nIn p198a(), eg9-7");

    use std::io::{self, Read};
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    for_p197b_to_p199b(read_username_from_file);
}

fn p199a() {
    println!("\nIn p199a(), eg9-8");

    use std::io::{self, Read};
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }

    for_p197b_to_p199b(read_username_from_file);
}

fn p199b() {
    println!("\nIn p199b(), eg9-9");

    use std::{fs, io};

    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    for_p197b_to_p199b(read_username_from_file);
}