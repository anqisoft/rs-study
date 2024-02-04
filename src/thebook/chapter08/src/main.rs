/*
 * Copyright (c) 2023 anqisoft@gmail.com
 * chapter08\src\main.rs
 *
 * https:∕∕kaisery.github.io∕trpl-zh-cn∕
 * https://kaisery.github.io/trpl-zh-cn/Rust%20%E7%A8%8B%E5%BA%8F%E8%AE%BE%E8%AE%A1%E8%AF%AD%E8%A8%80%20%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87%E7%89%88.pdf
 *
 * https://doc.rust-lang.org/stable/book/
 * https://doc.rust-lang.org/book/
 *
 * <en>
 * Created on Sun Nov 26 2023 13:57:30
 * Feature: Test the code of Chapter 08 in the Chinese version pdf.
 * The method name starts with p, followed by the PDF page number and the current page sub-number (a to z).
 * For example, p1a represents the first example on the first page of the PDF.
 * </en>
 *
 * <cn>
 * 创建：2023年11月26日 13:57:30
 * 功能：测试中文版pdf中第08章的代码。方法名以p开头，后面加pdf页序号及当页子序号（a到z），如p1a表示pdf第一页第一个例子。
 * </cn>
 *
 * <tw>
 * 創建：2023年11月26日 13:57:30
 * 功能：測試中文版pdf中第08章的程式碼。方法名稱以p開頭，後面加pdf頁序號及當頁子序號（a到z），如p1a表示pdf第一頁第一個例子。
 * </tw>
*/

// https://doc.rust-lang.org/book/ch08-01-vectors.html
// https://github.com/rust-lang/book/blob/main/src/ch08-01-vectors.md

fn p174a() {
    println!("\nIn p174a(), eg8-1");

    let _v1: Vec<i32> = Vec::new();
    let _v2: Vec<i32> = vec![];
}

fn p174b() {
    println!("\nIn p174b(), eg8-2");

    let _v = vec![1, 2, 3];
}

fn p174c() {
    println!("\nIn p174c(), eg8-3");

    let mut v1 = Vec::new();
    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    // error
    // let mut v2 = Vec::new();
    // v2.push(1);
    // v2.push(1.1);
    /*
        error[E0308]: mismatched types
            --> src\main.rs:56:13
            |
        55   |     v2.push(1);
            |     --      - this argument has type `{integer}`...
            |     |
            |     ... which causes `v2` to have type `Vec<{integer}>`
        56   |     v2.push(1.1);
            |        ---- ^^^ expected integer, found floating-point number
            |        |
            |        arguments to this method are incorrect
    */

    // error
    // let mut v3 = Vec::new();
    // v3.push(1.1);
    // v3.push(1);
    /*
        error[E0308]: mismatched types
            --> src\main.rs:74:13
            |
        73   |     v3.push(1.1);
            |     --      --- this argument has type `{float}`...
            |     |
            |     ... which causes `v3` to have type `Vec<{float}>`
        74   |     v3.push(1);
            |        ---- ^ expected floating-point number, found integer
            |        |
            |        arguments to this method are incorrect
            |
        note: method defined here
            --> D:\anqi\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\alloc\src\vec\mod.rs:1825:12
            |
        1825 |     pub fn push(&mut self, value: T) {
            |            ^^^^

        For more information about this error, try `rustc --explain E0308`.
    */

    let mut v4 = Vec::new();
    v4.push(1.1);
    println!("The type of v4 is {}.", type_of(&v4));
}

fn p175a() {
    println!("\nIn p175a(), eg8-4");

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    for i in 0..6 {
        // println!("The {i} element by index is {}", &v[i]);
        /*
            thread 'main' panicked at src\main.rs:116:54:
            index out of bounds: the len is 5 but the index is 5
        */

        if i == 0 {
            println!(
                "The type of first element by get({i}) is {}.",
                type_of(&v.get(i))
            );
        }

        match v.get(i) {
            Some(item) => println!("The {i} element by get() is {}", item),
            None => println!("Not found the {i} element."),
        }
    }

    for i in 0..5 {
        println!("The {i} element by index is {}", &v[i]);
    }
    println!("The type of first element by &v[0] is {}.", type_of(&v[0]));
}

fn p175b() {
    println!("\nIn p175b(), eg8-5");

    let v = vec![1, 2, 3, 4, 5];
    // error
    // let does_not_exist = &v[100];
    /*
        thread 'main' panicked at src\main.rs:146:28:
        index out of bounds: the len is 5 but the index is 100
    */

    let does_not_exist = v.get(100);
    println!("does_not_exist is {does_not_exist:?}");
}

fn p175c() {
    println!("\nIn p175c(), eg8-6");

    // // error
    // {
    //     let mut v = vec![1, 2, 3, 4, 5];
    //     let first = &v[0];

    //     v.push(6);
    //     println!("The first element is: {first}");
    // }
    /*
        error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
        --> src\main.rs:164:9
            |
        162 |         let first = &v[0];
            |                      - immutable borrow occurs here
        163 |
        164 |         v.push(6);
            |         ^^^^^^^^^ mutable borrow occurs here
        165 |         println!("The first element is: {first}");
            |                                         ------- immutable borrow later used here

        For more information about this error, try `rustc --explain E0502`.
    */

    // // ok
    // {
    //     let mut v = vec![1, 2, 3, 4, 5];
    //     let first = &v[0];

    //     println!("The first element is: {first}");
    // }
    /*
        warning: variable does not need to be mutable
        --> src\main.rs:184:13
            |
        184 |         let mut v = vec![1, 2, 3, 4, 5];
            |             ----^
            |             |
            |             help: remove this `mut`
            |
            = note: `#[warn(unused_mut)]` on by default

        warning: `chapter08` (bin "chapter08") generated 1 warning (run `cargo fix --bin "chapter08"` to apply 1 suggestion)
    */

    let v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    println!("The first element is: {first}");
}

fn p176a() {
    println!("\nIn p176a(), eg8-7");

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
}

fn p176b() {
    println!("\nIn p176b(), eg8-8");

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    println!("v is {v:?}");

    // error
    {
        let mut v = vec![100, 32, 57];
        for i in &v {
            println!("The type of {i} is {}", type_of(&i));

            // v.push(5);
            /*
                error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
                --> src\main.rs:234:13
                    |
                231 |         for i in &v {
                    |                  --
                    |                  |
                    |                  immutable borrow occurs here
                    |                  immutable borrow later used here
                ...
                234 |             v.push(5);
                    |             ^^^^^^^^^ mutable borrow occurs here

                For more information about this error, try `rustc --explain E0502`.
            */

            // if *i == 100 {
            //     v.pop();
            // }
            /*
                error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
                --> src\main.rs:252:17
                    |
                231 |         for i in &v {
                    |                  --
                    |                  |
                    |                  immutable borrow occurs here
                    |                  immutable borrow later used here
                ...
                252 |                 v.pop();
                    |                 ^^^^^^^ mutable borrow occurs here

                For more information about this error, try `rustc --explain E0502`.
            */
        }

        v.pop();
        println!("v is {v:?}");
        for i in &mut v {
            *i += 50;
            println!("The type of {i} is {}", type_of(&i));

            // v.push(5);
            /*
                error[E0499]: cannot borrow `v` as mutable more than once at a time
                --> src\main.rs:277:13
                    |
                273 |         for i in &mut v {
                    |                  ------
                    |                  |
                    |                  first mutable borrow occurs here
                    |                  first borrow later used here
                ...
                277 |             v.push(5);
                    |             ^ second mutable borrow occurs here

                For more information about this error, try `rustc --explain E0499`.
            */

            // if *i == 150 {
            //     v.pop();
            // }
            /*
                error[E0499]: cannot borrow `v` as mutable more than once at a time
                --> src\main.rs:295:17
                    |
                273 |         for i in &mut v {
                    |                  ------
                    |                  |
                    |                  first mutable borrow occurs here
                    |                  first borrow later used here
                ...
                295 |                 v.pop();
                    |                 ^ second mutable borrow occurs here

                For more information about this error, try `rustc --explain E0499`.
            */
        }

        let last_item = v.pop();
        println!(
            "The last item is {last_item:?}, and its type is {}.",
            type_of(&last_item)
        );
    }
}

fn p177a() {
    println!("\nIn p177a(), eg8-9");

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for item in row {
        println!("item is {item:?}");
    }
}

fn p178a() {
    println!("\nIn p178a(), eg8-10");

    let _v = vec![1, 2, 3, 4];
    // do stuff with _v
} // <- _v goes out of scope and is freed here

// https://doc.rust-lang.org/book/ch08-02-strings.html
// https://github.com/rust-lang/book/blob/main/src/ch08-02-strings.md

#[allow(unused_mut)]
#[allow(unused_variables)]
fn p179a() {
    println!("\nIn p179a(), eg8-11");

    let mut s = String::new();
}

fn p179b() {
    println!("\nIn p179b(), eg8-12");

    let data = "initial contents";
    let s1 = data.to_string();
    // the method also works on a literal directly:
    let s2 = "initial contents".to_string();
    println!("s1 == s2? {}.", &s1 == &s2);
}

fn p180a() {
    println!("\nIn p180a(), eg8-13");

    let s1 = String::from("initial contents");
    let s2 = "initial contents".to_string();
    println!("s1 == s2? {}.", &s1 == &s2);
}

fn p180b() {
    println!("\nIn p180b(), eg8-14");

    let hello = String::from("السلام عليكم");
    println!("'{hello}' has {} chars.", hello.chars().count());
    let hello = String::from("Dobrý den");
    println!("'{hello}' has {} chars.", hello.chars().count());
    let hello = String::from("Hello");
    println!("'{hello}' has {} chars.", hello.chars().count());
    let hello = String::from("שָׁלוֹם");
    println!("'{hello}' has {} chars.", hello.chars().count());
    let hello = String::from("नमस्ते");
    println!("'{hello}' has {} chars.", hello.chars().count());
    let hello = String::from("こんにちは");
    println!("'{hello}' has {} chars.", hello.chars().count());
    let hello = String::from("안녕하세요");
    println!("'{hello}' has {} chars.", hello.chars().count());
    let hello = String::from("你好");
    println!("'{hello}' has {} chars.", hello.chars().count());
    let hello = String::from("Olá");
    println!("'{hello}' has {} chars.", hello.chars().count());
    let hello = String::from("Здравствуйте");
    println!("'{hello}' has {} chars.", hello.chars().count());
    let hello = String::from("Hola");
    println!("'{hello}' has {} chars.", hello.chars().count());
}

fn p180c() {
    println!("\nIn p180c(), eg8-15");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s: {s}");
}

fn p181a() {
    println!("\nIn p181a(), eg8-16");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is '{s2}', and s1 is '{s1}'");
    // let ch = '!';
    // s1.push(ch);
    // println!("s2 is '{s2}', ch is '{ch}', and s1 is '{s1}'");
}

fn p181b() {
    println!("\nIn p181b(), eg8-17");

    let mut s = String::from("lo");
    s.push('l');
}

fn p181c() {
    println!("\nIn p181c(), eg8-18");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // note s1 has been moved here and can no longer be used
    println!("s3 is '{s3}, and s2 is '{s2}'");

    // error
    // println!("s1 is {s1}");
    /*
        error[E0382]: borrow of moved value: `s1`
        --> src\main.rs:443:21
            |
        436 |     let s1 = String::from("Hello, ");
            |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
        437 |     let s2 = String::from("world!");
        438 |     let s3 = s1 + &s2;
            |              -- value moved here
        ...
        443 |     println!("s1 is {s1}");
            |                     ^^^^ value borrowed here after move
            |
            = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
        help: consider cloning the value if the performance cost is acceptable
            |
        438 |     let s3 = s1.clone() + &s2;
            |                ++++++++

        For more information about this error, try `rustc --explain E0382`.
    */
}

fn p182a() {
    println!("\nIn p182a()");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s4 = &s1 + "-" + &s2 + "-" + &s3;
    /*
        error[E0369]: cannot add `&str` to `&String`
        --> src\main.rs:474:18
            |
        474 |     let s4 = &s1 + "-" + &s2 + "-" + &s3;
            |              --- ^ --- &str
            |              |   |
            |              |   `+` cannot be used to concatenate two `&str` strings
            |              &String
            |
            = note: string concatenation requires an owned `String` on the left
        help: remove the borrow to obtain an owned `String`
            |
        474 -     let s4 = &s1 + "-" + &s2 + "-" + &s3;
        474 +     let s4 = s1 + "-" + &s2 + "-" + &s3;
            |

        For more information about this error, try `rustc --explain E0369`.
    */

    let s5 = format!("{s1}-{s2}-{s3}");
    let s4 = s1 + "-" + &s2 + "-" + &s3;
    println!("s4 == s5? {}.", &s4 == &s5);
}

fn p182b() {
    println!("\nIn p182b(), eg8-19");

    // let s1 = String::from("hello");
    // let h = s1[0];
    // println!("h is '{h}'.");
}
/*
error[E0277]: the type `String` cannot be indexed by `{integer}`
   --> src\main.rs:504:13
    |
504 |     let h = s1[0];
    |             ^^^^^ `String` cannot be indexed by `{integer}`
    |
    = help: the trait `Index<{integer}>` is not implemented for `String`
    = help: the following other types implement trait `Index<Idx>`:
              <String as Index<RangeFull>>
              <String as Index<std::ops::Range<usize>>>
              <String as Index<RangeFrom<usize>>>
              <String as Index<RangeTo<usize>>>
              <String as Index<RangeInclusive<usize>>>
              <String as Index<RangeToInclusive<usize>>>

For more information about this error, try `rustc --explain E0277`.
*/

fn p183a() {
    println!("\nIn p183a()");

    let hello = String::from("السلام عليكم");
    println!(
        "'{hello}' has {} chars, and its length is {}.",
        hello.chars().count(),
        hello.len()
    );
    let hello = String::from("Dobrý den");
    println!(
        "'{hello}' has {} chars, and its length is {}.",
        hello.chars().count(),
        hello.len()
    );
    let hello = String::from("Hello");
    println!(
        "'{hello}' has {} chars, and its length is {}.",
        hello.chars().count(),
        hello.len()
    );
    let hello = String::from("שָׁלוֹם");
    println!(
        "'{hello}' has {} chars, and its length is {}.",
        hello.chars().count(),
        hello.len()
    );
    let hello = String::from("नमस्ते");
    println!(
        "'{hello}' has {} chars, and its length is {}.",
        hello.chars().count(),
        hello.len()
    );
    let hello = String::from("こんにちは");
    println!(
        "'{hello}' has {} chars, and its length is {}.",
        hello.chars().count(),
        hello.len()
    );
    let hello = String::from("안녕하세요");
    println!(
        "'{hello}' has {} chars, and its length is {}.",
        hello.chars().count(),
        hello.len()
    );
    let hello = String::from("你好");
    println!(
        "'{hello}' has {} chars, and its length is {}.",
        hello.chars().count(),
        hello.len()
    );
    let hello = String::from("Olá");
    println!(
        "'{hello}' has {} chars, and its length is {}.",
        hello.chars().count(),
        hello.len()
    );
    let hello = String::from("Здравствуйте");
    println!(
        "'{hello}' has {} chars, and its length is {}.",
        hello.chars().count(),
        hello.len()
    );
    let hello = String::from("Hola");
    println!(
        "'{hello}' has {} chars, and its length is {}.",
        hello.chars().count(),
        hello.len()
    );
}

fn p184a() {
    println!("\nIn p184a()");

    let s1 = String::from("hello");
    let h = &s1[0..1];
    println!("h is '{h}'.");

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s is '{s}'.");

    // let error_char = &hello[0..1];
    // println!("error_char is '{error_char}'.");
    /*
        thread 'main' panicked at src\main.rs:608:28:
        byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`
    */
}

fn p185a() {
    println!("\nIn p185a()");

    let s = "Зд".to_string();

    println!("The chars:");
    for c in s.chars() {
        println!("c is '{c}', and its type is {}.", type_of(&c));
    }

    println!("The bytes:");
    for b in s.bytes() {
        println!("b is {b}, and its type is {}.", type_of(&b));
    }

    println!("\nMore..");
    for str in [
        "السلام عليكم",
        "Dobrý den",
        "Hello",
        "שָׁלוֹם",
        "नमस्ते",
        "こんにちは",
        "안녕하세요",
        "你好",
        "Olá",
        "Здравствуйте",
        "Hola",
    ] {
        let s = str.to_string();
        println!("s is '{s}'");

        println!("The chars:");
        for c in s.chars() {
            println!("{c}");
        }

        println!("The bytes:");
        for b in s.bytes() {
            println!("{b}");
        }
    }
}

// https://doc.rust-lang.org/book/ch08-03-hash-maps.html
// https://github.com/rust-lang/book/blob/main/src/ch08-03-hash-maps.md
use std::collections::HashMap;

fn p186a() {
    println!("\nIn p186a(), eg8-20");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores is {scores:?}");
}

fn p186b() {
    println!("\nIn p186b()");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("The score of '{team_name}' team is {score}.");

    let new_name = "Red".to_string();
    let score = scores.get(&new_name).copied().unwrap_or(0);
    println!("The score of '{new_name}' team is {score}.");

    println!("The scores is {scores:?}");
}

// // ok
// fn p187a() {
//     println!("\nIn p187a()");

//     println!("Start...");
//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);
//     // This code will print each pair in an arbitrary order
//     for inner_index in 0..5 {
//         println!("\nThe {inner_index} time:");
//         for (key, value) in &scores {
//             print!("{key}: {value}\t");
//         }
//     }
// }

// error
// fn p187a() {
//     println!("\nIn p187a()");

//     for _outer_index in 0..3 {
//         println!("Start...");
//         let mut scores = HashMap::new();
//         scores.insert(String::from("Blue"), 10);
//         scores.insert(String::from("Yellow"), 50);

//         fn show_items() {
//             // This code will print each pair in an arbitrary order
//             for inner_index in 0..5 {
//                 println!("\nThe {inner_index} time:");
//                 for (key, value) in &scores {
//                     print!("{key}: {value}\t");
//                 }
//             }
//         }

//         println!("\n\nThe scores is {scores:?}");
//         show_items();
//     }
// }
/*
error[E0434]: can't capture dynamic environment in a fn item
   --> src\main.rs:724:38
    |
724 |                 for (key, value) in &scores {
    |                                      ^^^^^^
    |
    = help: use the `|| { ... }` closure form instead

For more information about this error, try `rustc --explain E0434`.
*/

fn p187a() {
    println!("\nIn p187a()");

    for _outer_index in 0..10 {
        println!("\n\nStart...");
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        fn show_items(scores: &HashMap<String, i32>) {
            // [anqisoft@gmail.com]It's not true: This code will print each pair in an arbitrary order
            for inner_index in 0..5 {
                print!("\nThe {inner_index} time:\t");
                for (key, value) in scores {
                    print!("{key}: {value}\t");
                }
            }
        }

        print!("The scores is {scores:?}");
        show_items(&scores);

        scores.insert(String::from("Orange"), 30);
        print!("\nThe scores is {scores:?}");
        show_items(&scores);

        scores.insert(String::from("Red"), 55);
        print!("\nThe scores is {scores:?}");
        show_items(&scores);
    }

    /*
     * <en>
     * The fact is: when we insert new items into a HashMap, the new items are inserted in a random order,
     * and the order of the old items may even be adjusted.
     * </en>
     *
     * <cn>事实是：当我们向 HashMap 插入新项时，新项插入顺序随机，甚至可能会调整旧项的顺序。</cn>
     * <tw>事實是：當我們向 HashMap 插入新項目時，新項目插入順序隨機，甚至可能會調整舊項的順序。</tw>
     */
}

fn p187b() {
    println!("\nIn p187b(), eg8-22");

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");
    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);

    // println!("map is {map:?}");

    // println!("field_name is '{field_name}', and field_value is '{field_value}'.");
    /*
        error[E0382]: borrow of moved value: `field_name`
        --> src\main.rs:798:30
            |
        791 |     let field_name = String::from("Favorite color");
            |         ---------- move occurs because `field_name` has type `String`, which does not implement the `Copy` trait
        ...
        794 |     map.insert(field_name, field_value);
            |                ---------- value moved here
        ...
        798 |     println!("field_name is '{field_name}', and field_value is '{field_value}'.");
            |                              ^^^^^^^^^^^^ value borrowed here after move
            |
            = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
        help: consider cloning the value if the performance cost is acceptable
            |
        794 |     map.insert(field_name.clone(), field_value);
            |                          ++++++++

        error[E0382]: borrow of moved value: `field_value`
        --> src\main.rs:798:65
            |
        792 |     let field_value = String::from("Blue");
            |         ----------- move occurs because `field_value` has type `String`, which does not implement the `Copy` trait
        793 |     let mut map = HashMap::new();
        794 |     map.insert(field_name, field_value);
            |                            ----------- value moved here
        ...
        798 |     println!("field_name is '{field_name}', and field_value is '{field_value}'.");
            |                                                                 ^^^^^^^^^^^^^ value borrowed here after move
            |
            = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
        help: consider cloning the value if the performance cost is acceptable
            |
        794 |     map.insert(field_name, field_value.clone());
            |                                       ++++++++

        For more information about this error, try `rustc --explain E0382`.
    */

    // ok
    {
        let field_name = "name".to_string();
        let field_value = 1;

        let mut map = HashMap::new();
        map.insert(field_name.clone(), field_value);
        println!("map is {map:?}, and its type is {}.", type_of(&map));
        println!("field_name is '{field_name}', and field_value is '{field_value}'.");
    }

    // ok
    {
        let field_name = "name";
        let field_value = 1;

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        println!("map is {map:?}, and its type is {}.", type_of(&map));
        println!("field_name is '{field_name}', and field_value is '{field_value}'.");
    }

    // ok
    {
        let mut field_name: Option<&str> = Some("name");
        let field_value = 1;

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        println!("map is {map:?}");
        println!("field_name is '{field_name:?}', and field_value is '{field_value}'.");

        field_name = None;
        println!("set field_name to None.");
        println!(
            "map is {map:?}, field_name is '{field_name:?}', and field_value is '{field_value}'."
        );
    }

    // ok
    {
        let mut s = String::from("name");
        let field_value = 1;
        let s1 = &mut s;

        let mut map = HashMap::new();
        map.insert(s1, field_value);
        println!("map is {:?}", &map);
        println!("map is {:?}", &map);

        s.push_str("!");

        // error
        // println!("map is {map:?}");
        /*
            error[E0499]: cannot borrow `s` as mutable more than once at a time
            --> src\main.rs:889:9
                |
            882 |         let s1 = &mut s;
                |                  ------ first mutable borrow occurs here
            ...
            889 |         s.push_str("!");
                |         ^ second mutable borrow occurs here
            ...
            892 |         println!("map is {map:?}");
                |                          ------- first borrow later used here

            For more information about this error, try `rustc --explain E0499`.
        */

        // error
        // println!("map is {:?}", &map);
        /*
            error[E0499]: cannot borrow `s` as mutable more than once at a time
            --> src\main.rs:889:9
                |
            882 |         let s1 = &mut s;
                |                  ------ first mutable borrow occurs here
            ...
            889 |         s.push_str("!");
                |         ^ second mutable borrow occurs here
            ...
            910 |         println!("map is {:?}", &map);
                |                                 ---- first borrow later used here

            For more information about this error, try `rustc --explain E0499`.
        */
    }
}

fn p188a() {
    println!("\nIn p188a(), eg8-23");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);
}

fn p188b() {
    println!("\nIn p188b(), eg8-24");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
}

fn p189a() {
    println!("\nIn p189a(), eg8-25");

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
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

            p174a();
            p174b();
            p174c();
            p175a();
            p175b();
            p175c();
            p176a();
            p176b();
            p177a();
            p178a();
            p179a();
            p179b();
            p180a();
            p180b();
            p180c();
            p181a();
            p181b();
            p181c();
            p182a();
            p182b();
            p183a();
            p184a();
            p185a();
            p186a();
            p186b();
            p187a();
            p187b();
            p188a();
            p188b();
            p189a();

            println!();
        };

        let functions = vec![
            ("p174a", p174a as fn()),
            ("p174b", p174b as fn()),
            ("p174c", p174c as fn()),
            ("p175a", p175a as fn()),
            ("p175b", p175b as fn()),
            ("p175c", p175c as fn()),
            ("p176a", p176a as fn()),
            ("p176b", p176b as fn()),
            ("p177a", p177a as fn()),
            ("p178a", p178a as fn()),
            ("p179a", p179a as fn()),
            ("p179b", p179b as fn()),
            ("p180a", p180a as fn()),
            ("p180b", p180b as fn()),
            ("p180c", p180c as fn()),
            ("p181a", p181a as fn()),
            ("p181b", p181b as fn()),
            ("p181c", p181c as fn()),
            ("p182a", p182a as fn()),
            ("p182b", p182b as fn()),
            ("p183a", p183a as fn()),
            ("p184a", p184a as fn()),
            ("p185a", p185a as fn()),
            ("p186a", p186a as fn()),
            ("p186b", p186b as fn()),
            ("p187a", p187a as fn()),
            ("p187b", p187b as fn()),
            ("p188a", p188a as fn()),
            ("p188b", p188b as fn()),
            ("p189a", p189a as fn()),
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
