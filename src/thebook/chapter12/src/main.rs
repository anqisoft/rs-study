/*
 * Copyright (c) 2023 anqisoft@gmail.com
 * chapter12\src\main.rs
 *
 * <en>https://doc.rust-lang.org/book/ch12-00-an-io-project.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch12-00-an-io-project.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch12-00-an-io-project.html</tw>
 *
 * <en>
 * Created on Tue Dec 05 2023 20:27:22
 * Feature: Test the code of Chapter 12 in the Chinese version pdf.
 * The method name starts with p, followed by the PDF page number and the current page sub-number (a to z).
 * For example, p1a represents the first example on the first page of the PDF.
 * </en>
 *
 * <cn>
 * 创建：2023年12月5日 20:27:22
 * 功能：测试中文版pdf中第12章的代码。方法名以p开头，后面加pdf页序号及当页子序号（a到z），如p1a表示pdf第一页第一个例子。
 * </cn>
 *
 * <tw>
 * 創建：2023年12月5日 20:27:22
 * 功能：測試中文版pdf中第12章的程式碼。方法名稱以p開頭，後面加pdf頁序號及當頁子序號（a到z），如p1a表示pdf第一頁第一個例子。
 * </tw>
*/

use std::env;

/*
* <en>https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch12-01-accepting-command-line-arguments.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch12-01-accepting-command-line-arguments.html</tw>
*/
fn p272a() {
    println!("\nIn p272a(), eg12-1");

    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
/* cargo run -- searchstring example-filename.txt
[src\main.rs:38] args = [
    "target\\debug\\minigrep.exe",
    "searchstring",
    "example-filename.txt",
]
*/
/* cargo run
[src\main.rs:38] args = [
    "target\\debug\\minigrep.exe",
]
*/
/* cargo run -- needle haystack
[src\main.rs:38] args = [
    "target\\debug\\minigrep.exe",
    "needle",
    "haystack",
]
*/

fn p273a() {
    println!("\nIn p273a(), eg12-2");

    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
/* cargo run -- searchstring example-filename.txt
Searching for searchstring
In file example-filename.txt
*/

/*
* <en>https://doc.rust-lang.org/book/ch12-02-reading-a-file.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch12-02-reading-a-file.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch12-02-reading-a-file.html</tw>
*/

/* eg12-3 poem.txt
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.
How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
*/
use std::fs;

fn p274a() {
    println!("\nIn p274a(), eg12-4");

    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}

/*
* <en>https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch12-03-improving-error-handling-and-modularity.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch12-03-improving-error-handling-and-modularity.html</tw>
*/

fn p277a() {
    println!("\nIn p277a(), eg12-5");

    fn parse_config(args: &[String]) -> (&str, &str) {
        let query = &args[1];
        let file_path = &args[2];
        (query, file_path)
    }

    let args: Vec<String> = env::args().collect();
    let (query, file_path) = parse_config(&args);

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}

fn p278a() {
    println!("\nIn p278a(), eg12-6");

    struct Config {
        query: String,
        file_path: String,
    }

    fn parse_config(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }

    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

fn p279a() {
    println!("\nIn p279a(), eg12-7");

    struct Config {
        query: String,
        file_path: String,
    }
    impl Config {
        fn new(args: &[String]) -> Self {
            let query = args[1].clone();
            let file_path = args[2].clone();
            Config { query, file_path }
        }
    }

    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

fn p280a() {
    println!("\nIn p280a(), eg12-8");

    struct Config {
        query: String,
        file_path: String,
    }
    impl Config {
        fn new(args: &[String]) -> Self {
            if args.len() < 3 {
                panic!("not enough arguments");
            }

            let query = args[1].clone();
            let file_path = args[2].clone();
            Config { query, file_path }
        }
    }

    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

fn p281a() {
    println!("\nIn p281a(), eg12-9");

    #[allow(dead_code)]
    struct Config {
        query: String,
        file_path: String,
    }

    impl Config {
        #[allow(dead_code)]
        fn build(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enough arguments");
            }
            let query = args[1].clone();
            let file_path = args[2].clone();
            Ok(Config { query, file_path })
        }
    }
}

use std::process;
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

fn p282a() {
    println!("\nIn p282a(), eg12-10");

    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

fn p284a() {
    println!("\nIn p284a(), eg12-11");

    fn run(config: Config) {
        let contents =
            fs::read_to_string(config.file_path).expect("Should have been able to read the file");

        println!("With text:\n{contents}");
    }

    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    run(config);
}

use std::error::Error;
fn p285a() {
    println!("\nIn p285a(), eg12-12");

    fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.file_path)?;
        println!("With text:\n{contents}");

        Ok(())
    }

    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let _ = run(config);
}

fn p287a() {
    println!("\nIn p287a()");

    fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.file_path)?;
        println!("With text:\n{contents}");

        Ok(())
    }

    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
    // match run(config) {
    //   Ok(()) => (),
    //   Err(error) => println!("The error is '{}'.", error)
    // };
}

fn p288a() {
    println!("\nIn p288a(), eg12-13");

    // see: lib.rs
}

fn p289a() {
    println!("\nIn p289a(), eg12-14");

    use minigrep::{run, Config};

    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {error}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

/*
* <en>https://doc.rust-lang.org/book/ch12-04-testing-the-librarys-functionality.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch12-04-testing-the-librarys-functionality.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch12-04-testing-the-librarys-functionality.html</tw>
*/

fn p290a() {
    println!("\nIn p290a(), eg12-15");

    // see: lib.rs, tests_eg12_15
}

fn p291a() {
    println!("\nIn p291a(), eg12-16");

    // see: lib.rs, tests_eg12_16
}

fn p294a() {
    println!("\nIn p294a(), eg12-17");

    // see: lib.rs, tests
}

fn p295a() {
    println!("\nIn p295a(), eg12-18");

    // see: lib.rs, tests
}

fn p296a() {
    println!("\nIn p296a(), eg12-19");

    // see: lib.rs, tests
}

fn p297a() {
    // println!("\nIn p297a()");

    use minigrep::{run, Config};

    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
/* cargo run -- frog poem.txt
How public, like a frog
*/
/* cargo run -- body poem.txt
I'm nobody! Who are you?
Are you nobody, too?
How dreary to be somebody!
*/
/* cargo run -- monomorphization poem.txt
*/

/*
* <en>https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch12-05-working-with-environment-variables.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch12-05-working-with-environment-variables.html</tw>
*/

fn p300a() {
    println!("\nIn p300a(), eg12-20");

    // see: lib.rs case_sensitive(), case_insensitive()
}

fn p302a() {
    println!("\nIn p302a(), eg12-21");

    // see: lib.rs search_case_insensitive()
}

fn p304a() {
    println!("\nIn p304a()");

    // see: lib.rs run_new()
}

fn p306a() {
    println!("\nIn p306a(), eg12-22");

    // see: lib.rs run_new()
}

fn p308a() {
    println!("\nIn p308a(), eg12-23");

    use minigrep::{run_new, ConfigNew};

    let args: Vec<String> = env::args().collect();
    let config = ConfigNew::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run_new(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

/*
* <en>https://doc.rust-lang.org/book/ch12-06-writing-to-stderr-instead-of-stdout.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch12-06-writing-to-stderr-instead-of-stdout.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch12-06-writing-to-stderr-instead-of-stdout.html</tw>
*/

fn p311a() {
    println!("\nIn p311a(), eg12-24");

    use minigrep::{run_new, ConfigNew};

    let args: Vec<String> = env::args().collect();
    let config = ConfigNew::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run_new(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

use std::time::Instant;
#[allow(unused_variables)]
fn main() {
    let main_function_name = "chapter12_main";
    let start_chapter_line = "Chapter 12";
    done_and_show_used_milliseconds(main_function_name, || {
        println!("start: {:?}", Instant::now());
        let action = || {
            println!("{start_chapter_line}");

            p272a();
            p273a();
            p274a();
            p277a();
            p278a();
            p279a();
            p280a();
            p281a();
            p282a();
            p284a();
            p285a();
            p287a();
            p288a();
            p289a();
            p290a();
            p291a();
            p294a();
            p295a();
            p296a();
            p297a();
            p300a();
            p302a();
            p304a();
            p306a();
            p308a();
            p311a();
        };

        let functions = vec![
            ("p272a", p272a as fn()),
            ("p273a", p273a as fn()),
            ("p274a", p274a as fn()),
            ("p277a", p277a as fn()),
            ("p278a", p278a as fn()),
            ("p279a", p279a as fn()),
            ("p280a", p280a as fn()),
            ("p281a", p281a as fn()),
            ("p282a", p282a as fn()),
            ("p284a", p284a as fn()),
            ("p285a", p285a as fn()),
            ("p287a", p287a as fn()),
            ("p288a", p288a as fn()),
            ("p289a", p289a as fn()),
            ("p290a", p290a as fn()),
            ("p291a", p291a as fn()),
            ("p294a", p294a as fn()),
            ("p295a", p295a as fn()),
            ("p296a", p296a as fn()),
            ("p297a", p297a as fn()),
            ("p300a", p300a as fn()),
            ("p302a", p302a as fn()),
            ("p304a", p304a as fn()),
            ("p306a", p306a as fn()),
            ("p308a", p308a as fn()),
            ("p311a", p311a as fn()),
        ];

        done_and_show_used_milliseconds(main_function_name, action);
        done_and_show_used_seconds(main_function_name, action);

        done_and_show_used_milliseconds_for_vec(functions.clone());
        done_and_show_used_seconds_for_vec(functions.clone());

        println!();
        println!("  end: {:?}", Instant::now());
    });
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
