/*
 * Copyright (c) 2023 anqisoft@gmail.com
 * chapter06\src\main.rs
 *
 * https:∕∕kaisery.github.io∕trpl-zh-cn∕
 * https://kaisery.github.io/trpl-zh-cn/Rust%20%E7%A8%8B%E5%BA%8F%E8%AE%BE%E8%AE%A1%E8%AF%AD%E8%A8%80%20%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87%E7%89%88.pdf
 *
 * https://doc.rust-lang.org/stable/book/
 * https://doc.rust-lang.org/book/
 *
 * <en>
 * Created on Mon Nov 27 2023 09:18:18
 * Feature: Test the code of Chapter 6 in the Chinese version pdf.
 * The method name starts with p, followed by the PDF page number and the current page sub-number (a to z).
 * For example, p1a represents the first example on the first page of the PDF.
 * </en>
 *
 * <cn>
 * 创建：2023年11月27日 09:18:18
 * 功能：测试中文版pdf中第六章的代码。方法名以p开头，后面加pdf页序号及当页子序号（a到z），如p1a表示pdf第一页第一个例子。
 * </cn>
 *
 * <tw>
 * 創建：2023年11月27日 09:18:18
 * 功能：測試中文版pdf中第六章的程式碼。方法名稱以p開頭，後面加pdf頁序號及當頁子序號（a到z），如p1a表示pdf第一頁第一個例子。
 * </tw>
*/

fn p133a() {
    println!("\nIn p133a()");

    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    {
        fn route(ip_kind: IpAddrKind) {
            println!("The ip kind is {ip_kind:?}.");
        }

        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        route(IpAddrKind::V4);
        route(IpAddrKind::V6);

        route(four);
        route(six);

        // error
        // println!("The type of four is {}", type_of(&four));
        // println!("The type of six is {}", type_of(&six));
        /*
            error[E0382]: borrow of moved value: `four`
            --> src\main.rs:53:52
            |
            43 |         let four = IpAddrKind::V4;
            |             ---- move occurs because `four` has type `p133a::IpAddrKind`, which does not implement the `Copy` trait
            ...
            49 |         route(four);
            |               ---- value moved here
            ...
            53 |         println!("The type of four is {}", type_of(&four));
            |                                                    ^^^^^ value borrowed here after move
            |
            note: consider changing this parameter type in function `route` to borrow instead if owning the value isn't necessary
            --> src\main.rs:39:27
            |
            39 |         fn route(ip_kind: IpAddrKind) {
            |            -----          ^^^^^^^^^^ this parameter takes ownership of the value
            |            |
            |            in this function

            error[E0382]: borrow of moved value: `six`
            --> src\main.rs:54:51
            |
            44 |         let six = IpAddrKind::V6;
            |             --- move occurs because `six` has type `p133a::IpAddrKind`, which does not implement the `Copy` trait
            ...
            50 |         route(six);
            |               --- value moved here
            ...
            54 |         println!("The type of six is {}", type_of(&six));
            |                                                   ^^^^ value borrowed here after move
            |
            note: consider changing this parameter type in function `route` to borrow instead if owning the value isn't necessary
            --> src\main.rs:39:27
            |
            39 |         fn route(ip_kind: IpAddrKind) {
            |            -----          ^^^^^^^^^^ this parameter takes ownership of the value
            |            |
            |            in this function

            For more information about this error, try `rustc --explain E0382`.
        */
    }

    {
        fn route(ip_kind: &IpAddrKind) {
            println!("The ip kind is {ip_kind:?}.");
        }

        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        route(&IpAddrKind::V4);
        route(&IpAddrKind::V6);

        route(&four);
        route(&six);

        println!("The type of four is {}", type_of(&four));
        println!("The type of six is {}", type_of(&six));
    }
}

fn p133b() {
    println!("\nIn p133b()");

    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    fn route(ip_kind: IpAddrKind, ip_address: &str) {
        println!("The ip kind is {ip_kind:?}, and its address is '{ip_address}'.");
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4, "127.0.0.1");
    route(IpAddrKind::V6, "::1");

    route(four, "127.0.0.1");
    route(six, "::1");
}

fn p135a() {
    println!("\nIn p135a()");

    // error
    // {
    //     enum IpAddrKind {
    //         V4,
    //         V6,
    //     }

    //     #[derive(Debug)]
    //     struct IpAddr {
    //         kind: IpAddrKind,
    //         address: String,
    //     }
    // }
    /*
        error[E0277]: `p135a::IpAddrKind` doesn't implement `Debug`
        --> src\main.rs:87:13
        |
        85 |         #[derive(Debug)]
        |                  ----- in this derive macro expansion
        86 |         struct IpAddr {
        87 |             kind: IpAddrKind,
        |             ^^^^^^^^^^^^^^^^ `p135a::IpAddrKind` cannot be formatted using `{:?}`
        |
        = help: the trait `Debug` is not implemented for `p135a::IpAddrKind`
        = note: add `#[derive(Debug)]` to `p135a::IpAddrKind` or manually `impl Debug for p135a::IpAddrKind`
        = note: this error originates in the derive macro `Debug` (in Nightly builds, run with -Z macro-backtrace for more info)
        help: consider annotating `p135a::IpAddrKind` with `#[derive(Debug)]`
        |
        80 +         #[derive(Debug)]
        81 |         enum IpAddrKind {
        |

        For more information about this error, try `rustc --explain E0277`.
    */

    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home: {home:?}.");
    println!("loopback: {loopback:?}.");

    println!("home: {home:#?}.");
    println!("loopback: {loopback:#?}.");

    dbg!(&home);
    dbg!(&loopback);

    println!("The type of home is {}", type_of(&home));
    println!("The type of loopback is {}", type_of(&loopback));
}

fn p135b() {
    println!("\nIn p135b()");

    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home: {home:?}.");
    println!("loopback: {loopback:?}.");
}

fn p136a() {
    println!("\nIn p136a()");

    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home: {home:?}.");
    println!("loopback: {loopback:?}.");
}

fn p136b() {
    println!("\nIn p136b()");

    #[derive(Debug)]
    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let quit_message = Message::Quit;
    let move_message = Message::Move { x: 10, y: 40 };
    let write_message = Message::Write(String::from("A write message"));
    let change_color_message = Message::ChangeColor(0x55, 0x55, 0x55);

    println!("kind: {}, value: {quit_message:?}", type_of(&quit_message));
    println!("kind: {}, value: {move_message:?}", type_of(&move_message));
    println!(
        "kind: {}, value: {write_message:?}",
        type_of(&write_message)
    );
    println!(
        "kind: {}, value: {change_color_message:?}",
        type_of(&change_color_message)
    );
}

fn p137a() {
    println!("\nIn p137a()");

    // unit-liked struct
    #[derive(Debug)]
    struct QuitMessage;

    // struct
    #[derive(Debug)]
    #[allow(dead_code)]
    struct MoveMessage {
        x: i32,
        y: i32,
    }

    // tuple struct
    #[derive(Debug)]
    struct WriteMessage(String);

    // tuple struct
    #[derive(Debug)]
    struct ChangeColorMessage(i32, i32, i32);

    let quit_message = QuitMessage;
    let move_message = MoveMessage { x: 10, y: 40 };
    let write_message = WriteMessage("A write message".to_string());
    let change_color_message = ChangeColorMessage(0x55, 0x55, 0x55);

    println!("kind: {}, value: {quit_message:?}", type_of(&quit_message));
    println!("kind: {}, value: {move_message:?}", type_of(&move_message));
    println!(
        "kind: {}, value: {write_message:?}",
        type_of(&write_message)
    );
    println!(
        "kind: {}, value: {change_color_message:?}",
        type_of(&change_color_message)
    );
}

fn p137b() {
    println!("\nIn p137b()");

    #[derive(Debug)]
    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            println!("kind: {}, value: {self:?}", type_of(&self));
        }
    }

    for message in [
        Message::Quit,
        Message::Move { x: 10, y: 40 },
        Message::Write(String::from("A write message")),
        Message::ChangeColor(0x55, 0x55, 0x55),
    ] {
        message.call();
    }
}

fn p138a() {
    println!("\nIn p138a()");

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    println!("kind: {}, value: {some_number:?}.", type_of(&some_number));
    println!("kind: {}, value: {some_char:?}.", type_of(&some_char));
    println!(
        "kind: {}, value: {absent_number:?}.",
        type_of(&absent_number)
    );
}

fn p138b() {
    println!("\nIn p138b()");

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // error
    // {
    //     let sum = x + y;
    // }
    /*
        error[E0277]: cannot add `Option<i8>` to `i8`
        --> src\main.rs:368:21
            |
        368 |         let sum = x + y;
            |                     ^ no implementation for `i8 + Option<i8>`
            |
            = help: the trait `Add<Option<i8>>` is not implemented for `i8`
            = help: the following other types implement trait `Add<Rhs>`:
                    <i8 as Add>
                    <i8 as Add<&i8>>
                    <&'a i8 as Add<i8>>
                    <&i8 as Add<&i8>>

        For more information about this error, try `rustc --explain E0277`.
    */

    // error
    // let sum = x + y.into::<i8>();
    // let sum: u8 = x + y.into();
    // let sum = x + (&y).into();

    // error
    // let _sum = x + match y {
    //     None => None,
    //     Some(value) => value,
    // };
    /*
        error[E0308]: `match` arms have incompatible types
        --> src\main.rs:395:24
            |
        393 |       let _sum = x + match y {
            |  ____________________-
        394 | |         None => None,
            | |                 ---- this is found to be of type `Option<_>`
        395 | |         Some(value) => value,
            | |                        ^^^^^ expected `Option<_>`, found `i8`
        396 | |     };
            | |_____- `match` arms have incompatible types
            |
            = note: expected enum `Option<_>`
                    found type `i8`
        help: try wrapping the expression in `Some`
            |
        395 |         Some(value) => Some(value),
            |                        +++++     +

        For more information about this error, try `rustc --explain E0308`.
    */

    let sum = x + match y {
        None => 0,
        Some(value) => value,
    };

    println!("x: {x}, its type is {}", type_of(&x));
    println!("y: {y:?}, its type is {}", type_of(&y));
    println!("sum: {sum}, its type is {}", type_of(&sum));
}

fn p140a() {
    println!("\nIn p140a()");

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: &Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    for coin in [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter] {
        println!(
            "coin is {} cents, and its type is {}.",
            value_in_cents(&coin),
            type_of(&coin)
        );
    }
}

fn p141a() {
    println!("\nIn p141a()");

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: &Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    for coin in [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter] {
        println!(
            "coin is {} cents, and its type is {}.",
            value_in_cents(&coin),
            type_of(&coin)
        );
    }
}

// Skipped p141b()

fn p142a() {
    println!("\nIn p142a()");

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: &Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                // println!("{state:?}");
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    for coin in [
        Coin::Penny,
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter(UsState::Alabama),
        Coin::Quarter(UsState::Alaska),
    ] {
        println!(
            "coin is {} cents, and its type is {}.",
            value_in_cents(&coin),
            type_of(&coin)
        );
    }
}

fn p142b() {
    println!("\nIn p142b()");

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(99) => {
                println!("It's 99.");
                Some(99 + 1)
            }
            Some(i) => Some(i + 1),
            // Some(98) => {
            //     println!("It's 98. This branch can never be reached.");
            //     Some(98 + 1)
            // }
            /*
                warning: unreachable pattern
                --> src\main.rs:550:13
                    |
                550 |             Some(98) => {
                    |             ^^^^^^^^
                    |
                    = note: `#[warn(unreachable_patterns)]` on by default

                warning: `chapter06` (bin "chapter06") generated 1 warning
            */
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let hundread = plus_one(Some(99));
    let ninty_nine = plus_one(Some(98));

    let none = plus_one(None);

    println!("five is {five:?}, six is {six:?}, ninty_nine is {ninty_nine:?}, hundread is {hundread:?}, and none is {none:?}.");
    println!("The type of five is {}", type_of(&five));
    println!("The type of six is {}", type_of(&six));
    println!("The type of none is {}", type_of(&none));
}

fn p144a() {
    println!("\nIn p144a()");

    // error
    // #[allow(dead_code)]
    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         Some(i) => Some(i + 1),
    //     }
    // }
    /*
        error[E0004]: non-exhaustive patterns: `None` not covered
        --> src\main.rs:587:15
            |
        587 |         match x {
            |               ^ pattern `None` not covered
            |
        note: `Option<i32>` defined here
        --> D:\anqi\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\option.rs:567:5
            |
        563 | pub enum Option<T> {
            | ------------------
        ...
        567 |     None,
            |     ^^^^ not covered
            = note: the matched value is of type `Option<i32>`
        help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
            |
        588 ~             Some(i) => Some(i + 1),
        589 ~             None => todo!(),
            |

        For more information about this error, try `rustc --explain E0004`.
    */
}

fn p145a() {
    println!("\nIn p145a()");

    fn add_fancy_hat() {
        println!("add_fancy_hat()");
    }
    fn remove_fancy_hat() {
        println!("remove_fancy_hat()");
    }
    fn move_player(num_spaces: u8) {
        println!("move_player({num_spaces})");
    }

    {
        let dice_roll = 9;
        println!("match dice_roll");
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            other => move_player(other),
        }
        println!();

        println!("loop from 1 to 10.");
        for n in 1..10 {
            match n {
                3 => add_fancy_hat(),
                7 => remove_fancy_hat(),
                other => move_player(other),
            }
        }
        println!();
    }

    println!("The value is assigned in a loop from 1 to 10 and the roll_dice method is called.");
    fn roll_dice(value: u8) {
        match value {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            other => move_player(other),
        }
    }
    for n in 1..10 {
        roll_dice(n);
    }
}

fn p145b() {
    println!("\nIn p145b()");

    fn add_fancy_hat() {
        println!("add_fancy_hat()");
    }
    fn remove_fancy_hat() {
        println!("remove_fancy_hat()");
    }
    fn reroll() {
        println!("reroll()");
    }

    fn roll_dice(value: u8) {
        match value {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => reroll(),
        }
    }
    for n in 1..10 {
        roll_dice(n);
    }
}

fn p145c() {
    println!("\nIn p145c()");

    fn add_fancy_hat() {
        println!("add_fancy_hat()");
    }
    fn remove_fancy_hat() {
        println!("remove_fancy_hat()");
    }

    fn roll_dice(value: u8) {
        match value {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),

            // Do nothing.
            _ => (),
        }
    }
    for n in 1..10 {
        roll_dice(n);
    }
}

fn p147a() {
    println!("\nIn p147a()");

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}

fn p147b() {
    println!("\nIn p147b()");

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn p147c() {
    println!("\nIn p147c()");

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    let mut count = 0;
    for coin in [
        Coin::Penny,
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter(UsState::Alabama),
        Coin::Quarter(UsState::Alaska),
    ] {
        match coin {
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
            }
            _ => count += 1,
        }
    }
    println!("The count of coin which isn't quarter is {count}");
}

fn p148a() {
    println!("\nIn p148a()");

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    let mut count = 0;
    for coin in [
        Coin::Penny,
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter(UsState::Alabama),
        Coin::Quarter(UsState::Alaska),
    ] {
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}", state);
        } else {
            count += 1;
        }
    }
    println!("The count of coin which isn't quarter is {count}");
}

use std::time::Instant;
#[allow(unused_variables)]
fn main() {
    let main_function_name = "chapter06_main";
    let start_chapter_line = "Chapter 06";
    done_and_show_used_milliseconds(main_function_name, || {
        println!("start: {:?}", Instant::now());
        let action = || {
            println!("{start_chapter_line}");

            p133a();
            p133b();

            p135a();
            p135b();

            p136a();
            p136b();

            p137a();
            p137b();

            p138a();
            p138b();

            p140a();

            p141a();

            p142a();
            p142b();

            p144a();

            p145a();
            p145b();
            p145c();

            p147a();
            p147b();
            p147c();

            p148a();
        };

        let functions = vec![
            ("p133a", p133a as fn()),
            ("p133b", p133b as fn()),
            ("p135a", p135a as fn()),
            ("p135b", p135b as fn()),
            ("p136a", p136a as fn()),
            ("p136b", p136b as fn()),
            ("p137a", p137a as fn()),
            ("p137b", p137b as fn()),
            ("p138a", p138a as fn()),
            ("p138b", p138b as fn()),
            ("p140a", p140a as fn()),
            ("p141a", p141a as fn()),
            ("p142a", p142a as fn()),
            ("p142b", p142b as fn()),
            ("p144a", p144a as fn()),
            ("p145a", p145a as fn()),
            ("p145b", p145b as fn()),
            ("p145c", p145c as fn()),
            ("p147a", p147a as fn()),
            ("p147b", p147b as fn()),
            ("p147c", p147c as fn()),
            ("p148a", p148a as fn()),
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
