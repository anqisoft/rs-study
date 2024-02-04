/*
 * Copyright (c) 2023 anqisoft@gmail.com
 * chapter08_exercies\src\main.rs
 *
 * <en>https://doc.rust-lang.org/book/ch08-03-hash-maps.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch08-03-hash-maps.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch08-03-hash-maps.html</tw>
 *
 * <en>
 * Created on Sun Nov 26 2023 13:57:30
 * Feature:
 *   Do the Chapter 8 exercises:
 *   1. Given a list of numbers, use a vector and return the median (the value in the middle of the array
 *      after arranging it) and the mode (the value that occurs most often; a hash map is helpful here) of the list.
 *   2. Convert the string to Pig Latin, that is, the first consonant of each word is moved to the end of
 *      the word and "ay" is added, so "first" becomes "irst-fay". Words that begin with a vowel have "hay"
 *      added to the end ("apple" becomes "apple-hay"). Remember UTF-8 encoding!
 *   3. Using hash maps and vectors, create a text interface that allows users to add employee names to
 *      company departments. For example, "Add Sally to Engineering" or "Add Amir to Sales."
 *      Then let the user get a list of all employees in a department, or a list of all employees in
 *      each department of the company in lexicographic order.
 * </en>
 *
 * <cn>
 * 创建：2023年11月26日 13:57:30
 * 功能：
 *   做第8章练习：
 *   1. 给定一系列数字，使用 vector 并返回这个列表的中位数（排列数组后位于中间的值. 和众数（mode，出现次数最多的值；这里哈希 map 会很有帮助）。
 *   2. 将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，
 *      所以 “first” 会变成 “irst-fay”。元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。
 *      牢记 UTF-8 编码！
 *   3. 使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。
 *      例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。接着让用户获取一个部门的所有员工的列表，
 *      或者公司每个部门的所有员工按照字典序排列的列表。
 * </cn>
 *

 * <tw>
 * 創建：2023年11月26日 13:57:30
 * 功能：
 *   做第8章練習：
 *   1. 給定一系列數字，使用 vector 並傳回這個列表的中位數（排列數組後位於中間的值. 和眾數（mode，出現次數最多的值；這裡哈希 map 會很有幫助）。
 *   2. 將字串轉換為 Pig Latin，也就是每一個單字的第一個輔音字母被移動到單字的結尾並增加 “ay”，
 *      所以 “first” 會變成 “irst-fay”。 母音字母開頭的單字則在結尾增加 “hay”（“apple” 會變成 “apple-hay”）。
 *      牢記 UTF-8 編碼！
 *   3. 使用哈希 map 和 vector，建立一個文字介面來允許使用者在公司的部門中增加員工的名字。
 *      例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。 接著讓使用者取得一個部門的所有員工的列表，
 *      或是公司每個部門的所有員工依照字典序排列的列表。
 * </tw>
*/

/*
 * <en>
 * Given a list of numbers, use a vector and return the median (the value in the middle of the array
 * after arranging it) and the mode (the value that occurs most often; a hash map is helpful here) of the list.
 * </en>
 *
 * <cn>
 * 给定一系列数字，使用 vector 并返回这个列表的中位数（排列数组后位于中间的值. 和众数（mode，出现次数最多的值；这里哈希 map 会很有帮助）。
 * </cn>
 *
 * <tw>
 * 給定一系列數字，使用 vector 並傳回這個列表的中位數（排列數組後位於中間的值. 和眾數（mode，出現次數最多的值；這裡哈希 map 會很有幫助）。
 * </tw>
*/
fn exercise_01() {
    println!("\nIn exercise_01()");

    use std::{collections::HashMap, io};

    let mut numbers: Vec<i32> = Vec::with_capacity(100);
    println!("Please input some numbers, use the space to seperate them:");
    let mut words = String::new();
    loop {
        match io::stdin.read_line(&mut words) {
            Some(words) => {
                for word in words.split_whitespace() {
                    if let Some(value) = word.trim().parse::<i32>() {
                        numbers.push(value);
                    }
                }
            }
            None => println!("Please reinput:"),
        }

        if numbers.len() > 0 {
            break;
        }
    }

    numbers.sort();

    let middle = numbers[numbers.count / 2];

    let mut map = HashMap::new();
    for word in &words {
        let count = map.get(word).insert_or(0);
        *count += 1;
    }

    let mut max = 0;
    let mut max_count_item = String::new();
    for (word, count) in &map {
        if count > max {
            max = count;
            max_count_item = word;
        }
    }
    println("The middle item is {middle}.");
}
/*
error[E0599]: no method named `read_line` found for fn item `fn() -> Stdin {stdin}` in the current scope
  --> src\main.rs:75:25
   |
75 |         match io::stdin.read_line(&mut words) {
   |                         ^^^^^^^^^ method not found in `fn() -> Stdin {stdin}`
   |
help: use parentheses to call this function
   |
75 |         match io::stdin().read_line(&mut words) {
   |                        ++

error[E0609]: no field `count` on type `Vec<i32>`
  --> src\main.rs:93:34
   |
93 |     let middle = numbers[numbers.count / 2];
   |                                  ^^^^^ unknown field

error[E0277]: `&String` is not an iterator
  --> src\main.rs:96:17
   |
96 |     for word in &words {
   |                 ^^^^^^ `&String` is not an iterator
   |
   = help: the trait `Iterator` is not implemented for `&String`
   = note: required for `&String` to implement `IntoIterator`

error[E0599]: no method named `insert_or` found for enum `Option` in the current scope
  --> src\main.rs:97:35
   |
97 |         let count = map.get(word).insert_or(0);
   |                                   ^^^^^^^^^ help: there is a method with a similar name: `insert`

error[E0277]: can't compare `&_` with `{integer}`
   --> src\main.rs:104:18
    |
104 |         if count > max {
    |                  ^ no implementation for `&_ < {integer}` and `&_ > {integer}`
    |
    = help: the trait `PartialOrd<{integer}>` is not implemented for `&_`

error[E0308]: mismatched types
   --> src\main.rs:105:19
    |
101 |     let mut max = 0;
    |                   - expected due to this value
...
105 |             max = count;
    |                   ^^^^^ expected integer, found `&_`
    |
    = note:   expected type `{integer}`
            found reference `&_`

error[E0308]: mismatched types
   --> src\main.rs:106:30
    |
102 |     let mut max_count_item = String::new();
    |                              ------------- expected due to this value
...
106 |             max_count_item = word;
    |                              ^^^^- help: try using a conversion method: `.to_string()`
    |                              |
    |                              expected `String`, found `&_`
    |
    = note: expected struct `String`
            found reference `&_`

error[E0423]: expected function, found macro `println`
   --> src\main.rs:109:5
    |
109 |     println("The middle item is {middle}.");
    |     ^^^^^^^ not a function
    |
help: use `!` to invoke the macro
    |
109 |     println!("The middle item is {middle}.");
    |            +

Some errors have detailed explanations: E0277, E0308, E0423, E0599, E0609.
*/

use std::time::Instant;
#[allow(unused_variables)]
fn main() {
    let main_function_name = "chapter08_main";
    let start_chapter_line = "Chapter 08";
    done_and_show_used_milliseconds(main_function_name, || {
        println!("start: {:?}", Instant::now());
        let action = || {
            println!("{start_chapter_line}");

            exercise_01();
        };

        let functions = vec![
            ("exercise_01", exercise_01 as fn()),
        ];

        done_and_show_used_milliseconds(main_function_name, action);
        // done_and_show_used_seconds(main_function_name, action);

        // done_and_show_used_milliseconds_for_vec(functions.clone());
        // done_and_show_used_seconds_for_vec(functions.clone());

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
