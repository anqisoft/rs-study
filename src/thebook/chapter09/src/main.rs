/*
 * Copyright (c) 2023 anqisoft@gmail.com
 * chapter09\src\main.rs
 *
 * <en>https://doc.rust-lang.org/book/ch09-00-error-handling.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch09-00-error-handling.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch09-00-error-handling.html</tw>
 *
 * <en>
 * Created on Fri Dec 01 2023 12:26:20
 * Feature: Test the code of Chapter 09 in the Chinese version pdf.
 * The method name starts with p, followed by the PDF page number and the current page sub-number (a to z).
 * For example, p1a represents the first example on the first page of the PDF.
 * </en>
 *
 * <cn>
 * 创建：2023年12月1日 12:26:20
 * 功能：测试中文版pdf中第09章的代码。方法名以p开头，后面加pdf页序号及当页子序号（a到z），如p1a表示pdf第一页第一个例子。
 * </cn>
 *
 * <tw>
 * 創建：2023年12月1日 12:26:20
 * 功能：測試中文版pdf中第09章的程式碼。方法名稱以p開頭，後面加pdf頁序號及當頁子序號（a到z），如p1a表示pdf第一頁第一個例子。
 * </tw>
*/

/* 9.1
    <en>https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html</en>
    <cn>https://kaisery.github.io/trpl-zh-cn/ch09-01-unrecoverable-errors-with-panic.html</cn>
    <tw>https://rust-lang.tw/book-tw/ch09-01-unrecoverable-errors-with-panic.html</tw>
*/

fn p192a() {
    println!("\nIn p192a()");

    // panic!("crash and burn");
}
/*
thread 'main' panicked at src\main.rs:36:5:
crash and burn
*/

fn p192b() {
    println!("\nIn p192b(), eg9-1");

    // let v = vec![1, 2, 3];
    // v[99];
}
/* eg-9.2
thread 'main' panicked at src\main.rs:47:6:
index out of bounds: the len is 3 but the index is 99
*/
/*
linux:
$ RUST_BACKTRACE=1 cargo run

Win:
SET RUST_BACKTRACE=1 && cls && cargo fmt && cargo run

thread 'main' panicked at src\main.rs:47:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0: std::panicking::begin_panic_handler
             at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library\std\src\panicking.rs:595
   1: core::panicking::panic_fmt
             at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library\core\src\panicking.rs:67
   2: core::panicking::panic_bounds_check
             at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33/library\core\src\panicking.rs:162
   3: core::slice::index::impl$2::index<i32>
             at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33\library\core\src\slice\index.rs:261
   4: alloc::vec::impl$12::index<i32,usize,alloc::alloc::Global>
             at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33\library\alloc\src\vec\mod.rs:2675
   5: chapter09::p192b
             at .\src\main.rs:47
   6: chapter09::main::closure$0::closure$0
             at .\src\main.rs:174
   7: chapter09::done_and_show_used_milliseconds<chapter09::main::closure$0::closure_env$0>
             at .\src\main.rs:230
   8: chapter09::main::closure$0
             at .\src\main.rs:216
   9: chapter09::done_and_show_used_milliseconds<chapter09::main::closure_env$0>
             at .\src\main.rs:230
  10: chapter09::main
             at .\src\main.rs:168
  11: core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
             at /rustc/cc66ad468955717ab92600c770da8c1601a4ff33\library\core\src\ops\function.rs:250
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
*/
/* Win:
SET RUST_BACKTRACE=full && cls && cargo fmt && cargo run
cls && SET RUST_BACKTRACE=full && cargo fmt && cargo run

Same as "SET RUST_BACKTRACE=1 && cls && Cargo fmt && Cargo Run" command.

REM Reset "RUST_BACKTRACE":
SET RUST_BACKTRACE=0
*/

/* 9.2
    <en>https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html</en>
    <cn>https://kaisery.github.io/trpl-zh-cn/ch09-02-recoverable-errors-with-result.html</cn>
    <tw>https://rust-lang.tw/book-tw/ch09-02-recoverable-errors-with-result.html</tw>
*/
use std::fs::File;
use std::io::ErrorKind;

fn p194a() {
    println!("\nIn p194a(), eg9-3");

    let greeting_file_result = File::open("hello.txt");
    // The type of greeting_file_result is core::result::Result<std::fs::File, std::io::error::Error>
    println!(
        "The type of greeting_file_result is {}",
        type_of(&greeting_file_result)
    );
}

fn p194b() {
    println!("\nIn p194b(), eg9-4");

    remove_hello_dot_txt_if_exists();
    // let greeting_file_result = File::open("hello.txt");
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };
}
/*
thread 'main' panicked at src\main.rs:123:23:
Problem opening the file: Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }
*/

fn remove_hello_dot_txt_if_exists() {
    const FILE_NAME: &str = "hello.txt";

    use std::fs;
    use std::path::Path;

    if Path::new(FILE_NAME).exists() {
        fs::remove_file(FILE_NAME).unwrap();
    }
}

#[allow(unused_variables)]
fn p195a() {
    println!("\nIn p195a(), eg9-5");

    const FILE_NAME: &str = "hello.txt";
    remove_hello_dot_txt_if_exists();

    let greeting_file_result = File::open(FILE_NAME);
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(FILE_NAME) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

#[allow(unused_variables)]
fn p196a() {
    println!("\nIn p196a(), same to eg9-5");

    const FILE_NAME: &str = "hello.txt";
    remove_hello_dot_txt_if_exists();

    let greeting_file = File::open(FILE_NAME).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(FILE_NAME).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

#[allow(unused_variables)]
fn p196b() {
    println!("\nIn p196b()");

    remove_hello_dot_txt_if_exists();
    // let greeting_file = File::open("hello.txt").unwrap();
}
/*
thread 'main' panicked at src\main.rs:189:49:
called `Result::unwrap()` on an `Err` value: Os {
    code: 2, kind: NotFound, message: "The system cannot find the file specified."
}
*/

#[allow(unused_variables)]
fn p197a() {
    println!("\nIn p197a()");

    remove_hello_dot_txt_if_exists();
    // let greeting_file =
    //     File::open("hello.txt").expect("hello.txt should be included in this project");
}
/*
thread 'main' panicked at src\main.rs:204:33:
hello.txt should be included in this project: Os {
    code: 2, kind: NotFound, message: "The system cannot find the file specified."
}
*/

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

fn p198a() {
    println!("\nIn p198a(), eg9-7");

    use std::io::{self, Read};
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

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

fn p199a() {
    println!("\nIn p199a(), eg9-8");

    use std::io::{self, Read};
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }

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

fn p199b() {
    println!("\nIn p199b(), eg9-9");

    use std::{fs, io};

    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

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

fn p199c() {
    println!("\nIn p199c(), eg9-10");

    // fn show_error_because_empty_tuple_incompatible() {
    //     let greeting_file = File::open("hello.txt")?;
    // }
    /*
        error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
        --> src\main.rs:327:52
            |
        326 |     fn show_error_because_empty_tuple_incompatible() {
            |     ------------------------------------------------ this function should return `Result` or `Option` to accept `?`
        327 |         let greeting_file = File::open("hello.txt")?;
            |                                                    ^ cannot use the `?` operator in a function that returns `()`
            |
            = help: the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `()`
    */
}

fn p200a() {
    println!("\nIn p200a(), eg9-11");

    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

    assert_eq!(
        last_char_of_first_line("Hello, world\nHow are you today?"),
        Some('d')
    );

    assert_eq!(last_char_of_first_line(""), None);
    assert_eq!(last_char_of_first_line("\nhi"), None);
}

fn p201a() {
    println!("\nIn p201a(), eg9-12");

    use std::error::Error;
    use std::io::Read;
    // I can use "main" as the internal function name. This is just an attempt, but it is not suitable!
    fn main() -> Result<(), Box<dyn Error>> {
        let mut greeting_file = File::open("hello.txt")?;
        let mut content = String::new();
        greeting_file.read_to_string(&mut content)?;
        println!("The content is '{}'", &content);
        Ok(())
    }

    println!("{:?}", main());
}

/*
    <en>
    9.3  To panic! or Not to panic!
    https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html
    </en>
    <cn>
    9.3  要不要 panic!
    https://kaisery.github.io/trpl-zh-cn/ch09-03-to-panic-or-not-to-panic.html
    </cn>
    <tw>
    9.3  要不要panic!
    https://rust-lang.tw/book-tw/ch09-03-to-panic-or-not-to-panic.html
    </tw>
*/

fn p202a() {
    println!("\nIn p202a()");

    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    println!("home ip address is {home:?}");
}

fn p203a() {
    println!("\nIn p203a()");

    use rand::Rng;
    use std::{cmp::Ordering, io};

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        // --snip--
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }
        match guess.cmp(&secret_number) {
            // --snip--
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn p204a() {
    println!("\nIn p204a(), eg9-13");

    mod sample {
        pub struct Guess {
            value: i32,
        }

        impl Guess {
            pub fn new(value: i32) -> Guess {
                if value < 1 || value > 100 {
                    panic!("Guess value must be between 1 and 100, got {}.", value);
                }
                Guess { value }
            }

            pub fn value(&self) -> i32 {
                self.value
            }
        }
    }

    use rand::Rng;
    use sample::Guess;
    use std::{cmp::Ordering, io};
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        // --snip--
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };

        match guess.value().cmp(&secret_number) {
            // --snip--
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

use std::time::Instant;
#[allow(unused_variables)]
fn main() {
    let main_function_name = "chapter09_main";
    let start_chapter_line = "Chapter 09";
    done_and_show_used_milliseconds(main_function_name, || {
        println!("start: {:?}", Instant::now());
        let action = || {
            println!("{start_chapter_line}");

            p192a();
            p192b();
            p194a();
            p194b();
            p195a();
            p196a();
            p196b();
            p197a();
            p197b();
            p198a();
            p199a();
            p199b();
            p199c();
            p200a();
            p201a();
            p202a();
            p203a();
            p204a();
        };

        let functions = vec![
            ("p192a", p192a as fn()),
            ("p192b", p192b as fn()),
            ("p194a", p194a as fn()),
            ("p194b", p194b as fn()),
            ("p195a", p195a as fn()),
            ("p196a", p196a as fn()),
            ("p196b", p196b as fn()),
            ("p197a", p197a as fn()),
            ("p197b", p197b as fn()),
            ("p198a", p198a as fn()),
            ("p199a", p199a as fn()),
            ("p199b", p199b as fn()),
            ("p199c", p199c as fn()),
            ("p200a", p200a as fn()),
            ("p201a", p201a as fn()),
            ("p202a", p202a as fn()),
            ("p203a", p203a as fn()),
            ("p204a", p204a as fn()),
        ];

        done_and_show_used_milliseconds(main_function_name, action);
        // done_and_show_used_seconds(main_function_name, action);

        // done_and_show_used_milliseconds_for_vec(functions.clone());
        // done_and_show_used_seconds_for_vec(functions.clone());

        println!();
        println!("  end: {:?}", Instant::now());
    });

    const ONE: u8 = 1;
    println!("The type of 1 is {}", type_of(&ONE));

    static TWO: f32 = 2.0;
    println!("The type of 2.0 is {}", type_of(&TWO));

    /*
        The type of 1 is u8
        The type of 2.0 is f32
    */
}

#[allow(dead_code)]
fn done_and_show_used_milliseconds(name: &str, func: impl Fn()) {
    let now = Instant::now();
    func();
    println!(
        "Calling {name} tooks {:?} milliseconds.",
        now.elapsed().as_millis()
    );
}

#[allow(dead_code)]
fn done_and_show_used_seconds(name: &str, func: impl Fn()) {
    let now = Instant::now();
    func();
    println!(
        "Calling {name} tooks {:?} seconds.",
        now.elapsed().as_secs()
    );
}

#[allow(dead_code)]
fn done_and_show_used_milliseconds_for_vec(functions: Vec<(&str, impl Fn())>) {
    let start = Instant::now();
    for (name, func) in functions {
        let now = Instant::now();
        func();
        println!(
            "Calling {name} tooks {:?} milliseconds.",
            now.elapsed().as_millis()
        );
    }
    println!("Total used {:?} milliseconds.", start.elapsed().as_millis());
}

#[allow(dead_code)]
fn done_and_show_used_seconds_for_vec(functions: Vec<(&str, impl Fn())>) {
    let start = Instant::now();
    for (name, func) in functions {
        let now = Instant::now();
        func();
        println!(
            "Calling {name} tooks {:?} seconds.",
            now.elapsed().as_secs()
        );
    }
    println!("Total used {:?} seconds.", start.elapsed().as_secs());
}

#[allow(dead_code)]
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
