/*
 * Copyright (c) 2023 anqisoft@gmail.com
 * chapter13\src\main.rs
 *
* <en>https://doc.rust-lang.org/book/ch13-00-functional-features.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch13-00-functional-features.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch13-00-functional-features.html</tw>
 *
 * <en>
 * Created on Wed Dec 06 2023 21:33:04
 * Feature: Test the code of Chapter 13 in the Chinese version pdf.
 * The method name starts with p, followed by the PDF page number and the current page sub-number (a to z).
 * For example, p1a represents the first example on the first page of the PDF.
 * </en>
 *
 * <cn>
 * 创建：2023年12月6日 21:33:04
 * 功能：测试中文版pdf中第13章的代码。方法名以p开头，后面加pdf页序号及当页子序号（a到z），如p1a表示pdf第一页第一个例子。
 * </cn>
 *
 * <tw>
 * 創建：2023年12月6日 21:33:04
 * 功能：測試中文版pdf中第13章的程式碼。方法名稱以p開頭，後面加pdf頁序號及當頁子序號（a到z），如p1a表示pdf第一頁第一個例子。
 * </tw>
*/

use std::{env, process, thread};
/*
* <en>https://doc.rust-lang.org/book/ch13-01-closures.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch13-01-closures.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch13-01-closures.html</tw>
*/

fn p316a() {
    println!("\nIn p316a(), eg13-1");

    #[derive(Debug, PartialEq, Copy, Clone)]
    enum ShirtColor {
        Red,
        Blue,
    }

    struct Inventory {
        shirts: Vec<ShirtColor>,
    }

    impl Inventory {
        fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
            user_preference.unwrap_or_else(|| self.most_stocked())
        }

        fn most_stocked(&self) -> ShirtColor {
            let mut num_red = 0;
            let mut num_blue = 0;

            for color in &self.shirts {
                match color {
                    ShirtColor::Red => num_red += 1,
                    ShirtColor::Blue => num_blue += 1,
                }
            }
            if num_red > num_blue {
                ShirtColor::Red
            } else {
                ShirtColor::Blue
            }
        }
    }

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

fn p318a() {
    println!("\nIn p318a(), eg13-2");

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    expensive_closure(1);
}

fn p319a() {
    println!("\nIn p319a(), eg13-3");

    let example_closure = |x| x;

    let _s = example_closure(String::from("hello"));

    // error
    // mismatched types expected `String`, found integer
    // try using a conversion method: `.to_string()`
    // let n = example_closure(5);

    let _n = example_closure(5.to_string());
}

fn p320a() {
    println!("\nIn p320a(), eg13-4");

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn p320b() {
    println!("\nIn p320b(), eg13-5");

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    // error
    //  println!("In calling closure: {:?}", list);
    /*
      error[E0502]: cannot borrow `list` as immutable because it is also borrowed as mutable
        --> src\main.rs:138:41
          |
      135 |    let mut borrows_mutably = || list.push(7);
          |                              -- ---- first borrow occurs due to use of `list` in closure
          |                              |
          |                              mutable borrow occurs here
      ...
      138 |    println!("In calling closure: {:?}", list);
          |                                         ^^^^ immutable borrow occurs here
      ...
      143 |    borrows_mutably();
          |    --------------- mutable borrow later used here
          |
          = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
    */

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

fn p321a() {
    println!("\nIn p321a(), eg13-6");

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn p322a() {
    println!("\nIn p322a(), eg13-7");

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}

fn p323a() {
    println!("\nIn p323a(), eg13-8");

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut sort_operations = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        // error
        // cannot move out of `value`, a captured variable in an `FnMut` closure
        // move occurs because `value` has type `String`, which does not implement the `Copy` trait
        // sort_operations.push(value);

        // ok
        sort_operations.push(value.clone());
        r.width
    });

    println!("{:#?}", list);
}

fn p324a() {
    println!("\nIn p324a(), eg13-9");

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}

/*
* <en>https://doc.rust-lang.org/book/ch13-02-iterators.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch13-02-iterators.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch13-02-iterators.html</tw>
*/

fn p325a() {
    println!("\nIn p325a(), eg13-10");

    let v1 = vec![1, 2, 3];

    let _v1_iter = v1.iter();
}

fn p325b() {
    println!("\nIn p325b(), eg13-11");

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

fn p326a() {
    println!("\nIn p326a(), eg13-12");

    // Need move to lib.rs
    // cannot test inner items
    // `#[warn(unnameable_test_items)]` on by default
    //  #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    iterator_demonstration();
}

fn p326b() {
    println!("\nIn p326b(), eg13-13");

    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    iterator_sum();
}

fn p327a() {
    println!("\nIn p327a(), eg13-14");

    let v1: Vec<i32> = vec![1, 2, 3];

    let _ = v1.iter().map(|x| x + 1);
}

fn p328a() {
    println!("\nIn p328a(), eg13-15");

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

fn p328b() {
    println!("\nIn p328b(), eg13-16");

    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    //  #[cfg(test)]
    //  mod tests {
    //      use super::*;

    //      #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
    // }

    filters_by_size();
}

/*
* <en>https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch13-03-improving-our-io-project.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch13-03-improving-our-io-project.html</tw>
*/

fn p330a() {
    println!("\nIn p330a(), eg13-17");

    #[allow(dead_code)]
    struct Config {
        query: String,
        file_path: String,
        ignore_case: bool,
    }

    impl Config {
        #[allow(dead_code)]
        pub fn build(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enough arguments");
            }

            let query = args[1].clone();
            let file_path = args[2].clone();

            let ignore_case = env::var("IGNORE_CASE").is_ok();

            Ok(Config {
                query,
                file_path,
                ignore_case,
            })
        }
    }

    // let args: Vec<String> = env::args().collect();
    //     let config = Config::build(&args).unwrap_or_else(|err| {
    //         eprintln!("Problem parsing arguments: {err}");
    //         process::exit(1);
    //     });
}

fn p332a() {
    println!("\nIn p332a()");

    //  let args: Vec<String> = env::args().collect();

    //  let config = Config::build(&args).unwrap_or_else(|err| {
    //      eprintln!("Problem parsing arguments: {err}");
    //      process::exit(1);
    //  });
}

fn p333a() {
    println!("\nIn p333a(), eg13-18");

    //  let config = Config::build(env::args()).unwrap_or_else(|err| {
    //   eprintln!("Problem parsing arguments: {err}");
    //   process::exit(1);
    // });
}

fn p333b() {
    println!("\nIn p333b(), eg13-19");

    // impl Config {
    //   pub fn build(
    //     mut args: impl Iterator<Item = String>,
    //   ) -> Result<Config, &'static str> {
    //   }
    // }
}

fn p335a() {
    println!("\nIn p335a(), eg13-20");

    #[allow(dead_code)]
    struct Config {
        query: String,
        file_path: String,
        ignore_case: bool,
    }

    impl Config {
        pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
            args.next();

            let query = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a query string"),
            };

            let file_path = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a file path"),
            };

            let ignore_case = env::var("IGNORE_CASE").is_ok();

            Ok(Config {
                query,
                file_path,
                ignore_case,
            })
        }
    }

    let _config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
}

// // // error:
// fn test_search<'a, T>(search: T)
// // where T: Fn<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
// where T: Fn() -> Vec<&'a str>
//  {
//   let query = "duct";
//   let contents = "\
//   Rust:
//   safe, fast, productive.
//   Pick three.";
//   assert_eq!(vec!["safe, fast, productive."], search(query, contents));
// }

fn p338a() {
    println!("\nIn p338a(), eg13-21");

    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();

        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }

        results
    }

    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";
    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}

fn p339a() {
    println!("\nIn p339a(), eg13-22");

    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    }

    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";
    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}

/*
* <en>https://doc.rust-lang.org/book/ch13-04-performance.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch13-04-performance.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch13-04-performance.html</tw>
*/

fn p342a() {
    println!("\nIn p342a()");

    // {
    //   let buffer: &mut [i32];
    //   let coefficients: [i64; 12];
    //   let qlp_shift: i16;

    //   for i in 12..buffer.len() {
    //       let prediction = coefficients.iter()
    //                                   .zip(&buffer[i - 12..i])
    //                                   .map(|(&c, &s)| c * s as i64)
    //                                   .sum::<i64>() >> qlp_shift;
    //       let delta = buffer[i];
    //       buffer[i] = prediction as i32 + delta;
    //   }
    // }
}

use std::time::{Duration, Instant};
#[allow(unused_variables)]
fn main() {
    let main_function_name = "chapter13_main";
    let start_chapter_line = "Chapter 13";
    done_and_show_used_milliseconds(main_function_name, || {
        println!("start: {:?}", Instant::now());
        let action = || {
            println!("{start_chapter_line}");

            p316a();
            p318a();
            p319a();
            p320a();
            p320b();
            p321a();
            p322a();
            p323a();
            p324a();
            p325a();
            p325b();
            p326a();
            p326b();
            p327a();
            p328a();
            p328b();
            p330a();
            p332a();
            p333a();
            p333b();
            p335a();
            p338a();
            p339a();
            p342a();
        };

        let functions = vec![
            ("p316a", p316a as fn()),
            ("p318a", p318a as fn()),
            ("p319a", p319a as fn()),
            ("p320a", p320a as fn()),
            ("p320b", p320b as fn()),
            ("p321a", p321a as fn()),
            ("p322a", p322a as fn()),
            ("p323a", p323a as fn()),
            ("p324a", p324a as fn()),
            ("p325a", p325a as fn()),
            ("p325b", p325b as fn()),
            ("p326a", p326a as fn()),
            ("p326b", p326b as fn()),
            ("p327a", p327a as fn()),
            ("p328a", p328a as fn()),
            ("p328b", p328b as fn()),
            ("p330a", p330a as fn()),
            ("p332a", p332a as fn()),
            ("p333a", p333a as fn()),
            ("p333b", p333b as fn()),
            ("p335a", p335a as fn()),
            ("p338a", p338a as fn()),
            ("p339a", p339a as fn()),
            ("p342a", p342a as fn()),
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
