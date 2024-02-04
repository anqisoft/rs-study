/*
 * Copyright (c) 2023 anqisoft@gmail.com
 * chapter18\src\main.rs
 * Reference Links:
 * <en>https://doc.rust-lang.org/book/ch18-00-patterns.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch18-00-patterns.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch18-00-patterns.html</tw>
*/

/* <en>
 * Created on Thu Dec 14 2023 09:02:03
 * Feature: Test the code of Chapter 18 in the Chinese version pdf.
 * The method name starts with p, followed by the PDF page number and the current page sub-number (a to z).
 * For example, p1a represents the first example on the first page of the PDF.
 * </en>
*/

/* <cn>
 * 创建：2023年12月14日 09:02:03
 * 功能：测试中文版pdf中第18章的代码。方法名以p开头，后面加pdf页序号及当页子序号（a到z），如p1a表示pdf第一页第一个例子。
 * </cn>
*/

/* <tw>
 * 創建：2023年12月14日 09:02:03
 * 功能：測試中文版pdf中第18章的程式碼。方法名稱以p開頭，後面加pdf頁序號及當頁子序號（a到z），如p1a表示pdf第一頁第一個例子。
 * </tw>
*/

/*
 * <en>https://doc.rust-lang.org/book/ch18-01-all-the-places-for-patterns.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch18-01-all-the-places-for-patterns.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch18-01-all-the-places-for-patterns.html</tw>
*/
fn eg1() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn eg2() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn eg3() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // after it.
    {
        // let PATTERN = EXPRESSION;
        let x = 5;
        println!("x is {x}");
    }
}

fn eg4() {
    let (x, y, z) = (1, 2, 3);
    println!("x is {x}, y is {y}, z is {z}");
}

fn eg5() {
    // error
    // mismatched types
    // expected tuple `({integer}, {integer}, {integer})`
    // found tuple `(_, _)`
    // let (x, y) = (1, 2, 3);
}

fn eg6() {
    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn foo(x: i32) {
        // code goes here
    }
}

fn eg7() {
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);
}

/*
 * <en>https://doc.rust-lang.org/book/ch18-02-refutability.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch18-02-refutability.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch18-02-refutability.html</tw>
*/
fn eg8() {
    let some_option_value = Some(1);

    // error
    // let Some(x) = some_option_value;
    /*
        refutable pattern in local binding
        `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
        for more information, visit https://doc.rust-lang.org/book/ch18-02-refutability.html
        the matched value is of type `Option<i32>`rustcClick for full compiler diagnostic
        main.rs(121, 36): you might want to use `let else` to handle the variant that isn't matched: ` else { todo!() }`
    */
    // use "let else"
    let Some(x) = some_option_value else { todo!() };
    println!("x is {x}");

    {
        let some_option_value = Some(1);
        let x = if let Some(x) = some_option_value {
            x
        } else {
            todo!()
        };
        println!("x is {x}");
    }
}

fn eg9() {
    let some_option_value = Some(1);
    if let Some(x) = some_option_value {
        println!("{}", x);
    }
}

fn eg10() {
    // if let x = 5 {
    //     println!("{}", x);
    // };
    /*
        irrefutable `if let` pattern
        this pattern will always match, so the `if let` is useless
        consider replacing the `if let` with a `let`
        `#[warn(irrefutable_let_patterns)]` on by defaultrustcClick for full compiler diagnostic
    */
    /*
        warning: irrefutable `if let` pattern
        --> src\main.rs:152:8
            |
        152 |     if let x = 5 {
            |        ^^^^^^^^^
            |
            = note: this pattern will always match, so the `if let` is useless
            = help: consider replacing the `if let` with a `let`
            = note: `#[warn(irrefutable_let_patterns)]` on by default
    */

    // after eg10
    {
        println!("\nAfter eg10:");
        let x = 1;

        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }
}

/*
 * <en>https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch18-03-pattern-syntax.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch18-03-pattern-syntax.html</tw>
*/

fn eg11() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    {
        println!("\nNext:");
        let x = 1;

        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    {
        println!("\nNext:");
        let x = 5;

        match x {
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }
    }

    {
        println!("\nNext:");
        let x = 'c';

        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }
}

fn eg12() {
    struct Point {
        x: i32,
        y: i32,
    }

    {
        let p = Point { x: 0, y: 7 };

        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);
    }
}

fn eg13() {
    struct Point {
        x: i32,
        y: i32,
    }

    {
        let p = Point { x: 0, y: 7 };

        let Point { x, y } = p;
        assert_eq!(0, x);
        assert_eq!(7, y);
    }
}

struct Point {
    x: i32,
    y: i32,
}
fn eg14() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}

fn eg15() {
    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    {
        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.");
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {x} and in the y direction {y}");
            }
            Message::Write(text) => {
                println!("Text message: {text}");
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {r}, green {g}, and blue {b}",)
            }
        }
    }
}

fn eg16() {
    #[allow(dead_code)]
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    {
        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change color to red {r}, green {g}, and blue {b}");
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change color to hue {h}, saturation {s}, value {v}")
            }
            _ => (),
        }
    }

    {
        println!("\nNext:");
        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
        println!("feet is {feet}, inches is {inches}, x is {x}, y is {y}");
    }
}

fn eg17() {
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    {
        foo(3, 4);
    }
}

fn eg18() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}

fn eg19() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
}

fn eg20() {
    {
        let _x = 5;
        let _y = 10;
    }
}

fn eg21() {
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    // println!("{:?}", s);
    /*
        error[E0382]: borrow of partially moved value: `s`
        --> src\main.rs:390:22
            |
        386 |     if let Some(_s) = s {
            |                 -- value partially moved here
        ...
        390 |     println!("{:?}", s);
            |                      ^ value borrowed here after partial move
            |
            = note: partial move occurs because value has type `String`, which does not implement the `Copy` trait
            = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
        help: borrow this binding in the pattern to avoid moving the value
            |
        386 |     if let Some(ref _s) = s {
            |                 +++

    */
}

fn eg22() {
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}

fn eg23() {
    #[allow(dead_code)]
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}

fn eg24() {
    {
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, .., last) => {
                println!("Some numbers: {first}, {last}");
            }
        }
    }
}

fn eg25() {
    {
        // let numbers = (2, 4, 8, 16, 32);

        // match numbers {
        //     (.., second, ..) => {
        //         println!("Some numbers: {}", second)
        //     }
        // }
    }
    /*
        error: `..` can only be used once per tuple pattern
        --> src\main.rs:452:26
            |
        452 |             (.., second, ..) => {
            |              --          ^^ can only be used once per tuple pattern
            |              |
            |              previously used here
    */
}

fn eg26() {
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
}

fn eg27() {
    {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("Matched, n = {n}"),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {y}", x);
    }
}

fn eg28() {
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn eg29() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}

fn main() {
    for (name, f) in [
        ("eg1", eg1 as fn()),
        ("eg2", eg2 as fn()),
        ("eg3", eg3 as fn()),
        ("eg4", eg4 as fn()),
        ("eg5", eg5 as fn()),
        ("eg6", eg6 as fn()),
        ("eg7", eg7 as fn()),
        ("eg8", eg8 as fn()),
        ("eg9", eg9 as fn()),
        ("eg10", eg10 as fn()),
        ("eg11", eg11 as fn()),
        ("eg12", eg12 as fn()),
        ("eg13", eg13 as fn()),
        ("eg14", eg14 as fn()),
        ("eg15", eg15 as fn()),
        ("eg16", eg16 as fn()),
        ("eg17", eg17 as fn()),
        ("eg18", eg18 as fn()),
        ("eg19", eg19 as fn()),
        ("eg20", eg20 as fn()),
        ("eg21", eg21 as fn()),
        ("eg22", eg22 as fn()),
        ("eg23", eg23 as fn()),
        ("eg24", eg24 as fn()),
        ("eg25", eg25 as fn()),
        ("eg26", eg26 as fn()),
        ("eg27", eg27 as fn()),
        ("eg28", eg28 as fn()),
        ("eg29", eg29 as fn()),
    ] {
        println!("\nIn {}():", name);
        f();
    }
}
