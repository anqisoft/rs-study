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
        match io::stdin().read_line(&mut words) {
            Ok(_byte_count) => {
                for word in words[..].split_whitespace() {
                    if let Ok(value) = word.trim().parse::<i32>() {
                        numbers.push(value);
                    }
                }
            }
            Err(info) => println!("The error is '{info}', please reinput:"),
        }

        if numbers.len() > 0 {
            break;
        }
    }

    numbers.sort();

    let median = numbers[numbers.len() / 2];

    let mut map = HashMap::new();
    for number in &numbers {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }

    let mut max_count = 0;
    let mut max_count_item = 0;
    for (number, count) in &map {
        if *count > max_count {
            max_count = *count;
            max_count_item = **number;
        }
    }
    println!("The median is {median}, and the mode is {max_count_item}, its count is {max_count}.");
}
/*
In exercise_01()
Please input some numbers, use the space to seperate them:
2 5 7 4 8 9 6 3 4 5 1 2 2 6 7 8 9
The median is 5, and the mode is 2, its count is 3.
*/

/*
 * <en>
 * Convert the string to Pig Latin, that is, the first consonant of each word is moved to the end of
 * the word and "ay" is added, so "first" becomes "irst-fay". Words that begin with a vowel have "hay"
 * added to the end ("apple" becomes "apple-hay"). Remember UTF-8 encoding!
 * </en>
 *
 * <cn>
 * 将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，
 * 所以 “first” 会变成 “irst-fay”。元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。
 * 牢记 UTF-8 编码！
 * </cn>
 *
 * <tw>
 * 將字串轉換為 Pig Latin，也就是每一個單字的第一個輔音字母被移動到單字的結尾並增加 “ay”，
 * 所以 “first” 會變成 “irst-fay”。 母音字母開頭的單字則在結尾增加 “hay”（“apple” 會變成 “apple-hay”）。
 * 牢記 UTF-8 編碼！
 * </tw>
*/
fn exercise_02() {
    println!("\nIn exercise_02()");

    fn convert_to_pig_latin(from: &str) -> String {
        let from_chars = from.chars();
        let first_char = from_chars[0];
        if "aeiou".contains(first_char) {
            from + "-hay"
        } else {
            format!("{}-{first_char}ay", from_chars.unshift().join(""));
        }
    }

    for word in ["first", "apple"] {
        println!("Converted '{}' to '{}'", word, convert_to_pig_latin(word));
    }
}
/*
error[E0608]: cannot index into a value of type `Chars<'_>`
   --> src\main.rs:142:36
    |
142 |         let first_char = from_chars[0];
    |                                    ^^^

error[E0369]: cannot add `&str` to `&str`
   --> src\main.rs:144:18
    |
144 |             from + "-hay"
    |             ---- ^ ------ &str
    |             |    |
    |             |    `+` cannot be used to concatenate two `&str` strings
    |             &str
    |
    = note: string concatenation requires an owned `String` on the left
help: create an owned `String` from a string reference
    |
144 |             from.to_owned() + "-hay"
    |                 +++++++++++

error[E0599]: no method named `unshift` found for struct `Chars` in the current scope
   --> src\main.rs:146:53
    |
146 |             format!("{}-{first_char}ay", from_chars.unshift().join(""));
    |                                                     ^^^^^^^ method not found in `Chars<'_>`

error[E0308]: mismatched types
   --> src\main.rs:145:16
    |
145 |           } else {
    |  ________________^
146 | |             format!("{}-{first_char}ay", from_chars.unshift().join(""));
    | |                                                                        - help: remove this semicolon to return this value
147 | |         }
    | |_________^ expected `String`, found `()`

Some errors have detailed explanations: E0308, E0369, E0599, E0608.
*/

/*
 * <en>
 * Using hash maps and vectors, create a text interface that allows users to add employee names to
 * company departments. For example, "Add Sally to Engineering" or "Add Amir to Sales."
 * Then let the user get a list of all employees in a department, or a list of all employees in
 * each department of the company in lexicographic order.
 * </en>
 *
 * <cn>
 * 使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。
 * 例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。接着让用户获取一个部门的所有员工的列表，
 * 或者公司每个部门的所有员工按照字典序排列的列表。
 * </cn>
 *
 * <tw>
 * 使用哈希 map 和 vector，建立一個文字介面來允許使用者在公司的部門中增加員工的名字。
 * 例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。 接著讓使用者取得一個部門的所有員工的列表，
 * 或是公司每個部門的所有員工依照字典序排列的列表。
 * </tw>
*/
fn exercise_03() {
    // println!("\nIn exercise_03()");
    //
    //
}

use std::time::Instant;
#[allow(unused_variables)]
fn main() {
    let main_function_name = "chapter08_main";
    let start_chapter_line = "Chapter 08";
    done_and_show_used_milliseconds(main_function_name, || {
        println!("start: {:?}", Instant::now());
        let action = || {
            println!("{start_chapter_line}");

            // exercise_01();
            exercise_02();
            exercise_03();
        };

        let functions = vec![
            ("exercise_01", exercise_01 as fn()),
            ("exercise_02", exercise_02 as fn()),
            ("exercise_03", exercise_03 as fn()),
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
