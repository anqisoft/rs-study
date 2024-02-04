/*
 * Copyright (c) 2023 anqisoft@gmail.com
 * chapter05\src\main.rs
 *
 * https:∕∕kaisery.github.io∕trpl-zh-cn∕
 * https://kaisery.github.io/trpl-zh-cn/Rust%20%E7%A8%8B%E5%BA%8F%E8%AE%BE%E8%AE%A1%E8%AF%AD%E8%A8%80%20%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87%E7%89%88.pdf
 *
 * https://doc.rust-lang.org/stable/book/
 * https://doc.rust-lang.org/book/
 *
 * <en>
 * Created on Mon Nov 27 2023 19:40:35
 * Feature: Test the code of Chapter 7 in the Chinese version pdf.
 * The method name starts with p, followed by the PDF page number and the current page sub-number (a to z).
 * For example, p1a represents the first example on the first page of the PDF.
 * </en>
 *
 * <cn>
 * 创建：2023年11月27日 19:40:35
 * 功能：测试中文版pdf中第7章的代码。方法名以p开头，后面加pdf页序号及当页子序号（a到z），如p1a表示pdf第一页第一个例子。
 * </cn>
 *
 * <tw>
 * 創建：2023年11月27日 19:40:35
 * 功能：測試中文版pdf中第7章的程式碼。方法名稱以p開頭，後面加pdf頁序號及當頁子序號（a到z），如p1a表示pdf第一頁第一個例子。
 * </tw>
*/

// fn p154a() {
//     println!("\nIn p154a()");

//     use crate::garden::vegetables::Asparagus;

//     // Why not "mod garden;"?
//     pub mod garden;

//     let plant = Asparagus {};
//     println!("I'm growing {:?}!", plant);
// }
/*
error: cannot declare a non-inline module inside a block unless it has a path attribute
  --> src\main.rs:35:5
   |
35 |     pub mod garden;
   |     ^^^^^^^^^^^^^^^
   |
note: maybe `use` the module `garden` instead of redeclaring it
  --> src\main.rs:35:5
   |
35 |     pub mod garden;
   |     ^^^^^^^^^^^^^^^

error[E0433]: failed to resolve: could not find `garden` in the crate root
  --> src\main.rs:32:16
   |
32 |     use crate::garden::vegetables::Asparagus;
   |                ^^^^^^ could not find `garden` in the crate root

For more information about this error, try `rustc --explain E0433`.
*/

// // ok
// use crate::garden::vegetables::Asparagus;
// // Why not "mod garden;"?
// pub mod garden;
// fn p154a() {
//     println!("\nIn p154a()");

//     let plant = Asparagus {};
//     println!("I'm growing {:?}!", plant);
// }

// ok
use crate::garden::vegetables::Asparagus;
mod garden;
fn p154a() {
    println!("\nIn p154a()");

    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}

// // error
// use crate::garden::vegetables::Asparagus;
// use garden;
// fn p154a() {
//     println!("\nIn p154a()");

//     let plant = Asparagus {};
//     println!("I'm growing {:?}!", plant);
// }
/*
error[E0432]: unresolved import `garden`
  --> src\main.rs:85:5
   |
85 | use garden;
   |     ^^^^^^ no external crate `garden`

For more information about this error, try `rustc --explain E0432`.
*/

fn p164a() {
    println!("\nIn p164a()");

    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(1, 2);

    println!("map is {map:?}, and its type is {}", type_of(&map));
}

fn p166a() {
    println!("\nIn p166a()");

    // see: chapter2
    use rand::Rng;
    use std::io;

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}

fn p166b() {
    println!("\nIn p166b()");

    // See: eg2-4
    use rand::Rng;
    // --snip--

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    let mut guess = String::new();

    // Use two statements:
    {
        println!("use std::cmp::Ordering;use std::io;");
        use std::cmp::Ordering;
        use std::io;

        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");
        let guess = match guess.trim().parse::<u32>() {
            Ok(value) => value,
            Err(_) => 0, // expect("")
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }

    // Use one statements:
    {
        println!("use std::{{ cmp::Ordering, io}};");
        use std::{cmp::Ordering, io};

        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");
        let guess = match guess.trim().parse::<u32>() {
            Ok(value) => value,
            Err(_) => 0, // expect("")
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}

use std::time::Instant;
#[allow(unused_variables)]
fn main() {
    let main_function_name = "chapter00_main";
    let start_chapter_line = "Chapter 00";
    done_and_show_used_milliseconds(main_function_name, || {
        println!("start: {:?}", Instant::now());
        let action = || {
            println!("{start_chapter_line}");

            p154a();

            p164a();

            p166a();
            p166b();
        };

        let functions = vec![
            ("p154a", p154a as fn()),
            ("p164a", p164a as fn()),
            ("p166a", p166a as fn()),
            ("p166b", p166b as fn()),
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
