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
 * Created on Sun Nov 26 2023 13:57:30
 * Feature: Test the code of Chapter 5 in the Chinese version pdf.
 * The method name starts with p, followed by the PDF page number and the current page sub-number (a to z).
 * For example, p1a represents the first example on the first page of the PDF.
 * </en>
 *
 * <cn>
 * 创建：2023年11月26日 13:57:30
 * 功能：测试中文版pdf中第五章的代码。方法名以p开头，后面加pdf页序号及当页子序号（a到z），如p1a表示pdf第一页第一个例子。
 * </cn>
 *
 * <tw>
 * 創建：2023年11月26日 13:57:30
 * 功能：測試中文版pdf中第五章的程式碼。方法名稱以p開頭，後面加pdf頁序號及當頁子序號（a到z），如p1a表示pdf第一頁第一個例子。
 * </tw>
*/

#[allow(dead_code)]
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
#[warn(dead_code)]

fn p114a() {
    println!("\nIn p114a()");

    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!(
        "The type of user1 is {}, and its value is {user1:#?}.",
        type_of(&user1)
    );
}

fn p114b() {
    println!("\nIn p114b()");

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!(
        "The user1.email is '{}', and its type of {}.",
        user1.email,
        type_of(&user1.email)
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
fn p115a() {
    println!("\nIn p115a()");

    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    println!("user1: {user1:#?}.");
}

fn p115b() {
    println!("\nIn p115b()");

    fn build_user_simple(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
    let user1 = build_user_simple(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    println!("user1: {user1:#?}.");
}

fn p116a() {
    println!("\nIn p116a()");

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("user1: {user1:#?}.");

    // error
    // {
    //     let user2 = User {
    //         active: user1.active,
    //         username: user1.username,
    //         email: String::from("another@example.com"),
    //         sign_in_count: user1.sign_in_count,
    //     };
    //     println!("user2: {user2:#?}.");

    //     let user3 = User {
    //         email: String::from("another@example.com"),
    //         ..user1
    //     };
    //     println!("user3: {user3:#?}.");
    // }
    /*
        error[E0382]: use of moved value: `user1.username`
        --> src\main.rs:130:21
            |
        124 |               username: user1.username,
            |                         -------------- value moved here
        ...
        130 |           let user3 = User {
            |  _____________________^
        131 | |             email: String::from("another@example.com"),
        132 | |             ..user1
        133 | |         };
            | |_________^ value used here after move
            |
            = note: move occurs because `user1.username` has type `String`, which does not implement the `Copy` trait

        For more information about this error, try `rustc --explain E0382`.
    */

    // It's ok, when I change "username: user1.username" to "username: user1.username.clone()".
    let user2 = User {
        active: user1.active,
        username: user1.username.clone(),
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("user2: {user2:#?}.");

    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("user3: {user3:#?}.");

    // println!("user1: {user1:#?}.");
}

fn p117a() {
    println!("\nIn p117a()");

    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Type: {}, the black is '{black:?}'.", type_of(&black));
    println!("Type: {}, the origin is '{origin:?}'.", type_of(&origin));

    println!("The origin is '{origin:?}' or {origin:#?}");
}

fn p118a() {
    println!("\nIn p118a()");

    #[derive(Debug, PartialEq)]
    struct AlwaysEqual;

    let a = AlwaysEqual;
    let b = AlwaysEqual;

    println!("Is a same to b? {}.", if a == b { "yes" } else { "no" });

    println!("a is {a:?}, and its type is {}.", type_of(&a));
    println!("b is {b:?}, and its type is {}.", type_of(&b));
}

fn p118b() {
    println!("\nIn p118b()");

    // struct User {
    //     active: bool,
    //     username: &str,
    //     email: &str,
    //     sign_in_count: u64,
    // }

    // let _user1 = User {
    //     active: true,
    //     username: "someusername123",
    //     email: "someone@example.com",
    //     sign_in_count: 1,
    // };
}
/*
error[E0106]: missing lifetime specifier
   --> src\main.rs:209:19
    |
209 |         username: &str,
    |                   ^ expected named lifetime parameter
    |
help: consider introducing a named lifetime parameter
    |
207 ~     struct User<'a> {
208 |         active: bool,
209 ~         username: &'a str,
    |

error[E0106]: missing lifetime specifier
   --> src\main.rs:210:16
    |
210 |         email: &str,
    |                ^ expected named lifetime parameter
    |
help: consider introducing a named lifetime parameter
    |
207 ~     struct User<'a> {
208 |         active: bool,
209 |         username: &str,
210 ~         email: &'a str,
    |

For more information about this error, try `rustc --explain E0106`.
*/

fn p120a() {
    println!("\nIn p120a()");

    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
}

fn p121a() {
    println!("\nIn p121a()");

    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );

    fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
}

fn p121b() {
    println!("\nIn p121b()");

    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

#[allow(unused_variables)]
fn p122a() {
    println!("\nIn p122a()");

    #[allow(dead_code)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // error
    // {
    //     println!("rect1 is {}", rect1);
    // }
    /*
        error[E0277]: `p122a::Rectangle` doesn't implement `std::fmt::Display`
        --> src\main.rs:318:33
            |
        318 |         println!("rect1 is {}", rect1);
            |                                 ^^^^^ `p122a::Rectangle` cannot be formatted with the default formatter
            |
            = help: the trait `std::fmt::Display` is not implemented for `p122a::Rectangle`
            = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
            = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

        For more information about this error, try `rustc --explain E0277`.
    */

    // error
    // {
    //     println!("rect1 is {:?}", rect1);
    // }
    /*
        error[E0277]: `p122a::Rectangle` doesn't implement `Debug`
        --> src\main.rs:336:35
            |
        336 |         println!("rect1 is {:?}", rect1);
            |                                   ^^^^^ `p122a::Rectangle` cannot be formatted using `{:?}`
            |
            = help: the trait `Debug` is not implemented for `p122a::Rectangle`
            = note: add `#[derive(Debug)]` to `p122a::Rectangle` or manually `impl Debug for p122a::Rectangle`
            = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
        help: consider annotating `p122a::Rectangle` with `#[derive(Debug)]`
            |
        306 +     #[derive(Debug)]
        307 |     struct Rectangle {
            |

        For more information about this error, try `rustc --explain E0277`.
    */
}

fn p123a() {
    println!("\nIn p123a()");

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
}

fn p124a() {
    println!("\nIn p124a()");

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
}
/*
In p124a()
[src\main.rs:391] 30 * scale = 60
[src\main.rs:394] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
*/

fn p125a() {
    println!("\nIn p125a()");

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

fn p126a() {
    println!("\nIn p126a()");

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        /*
            <en>It is not a getter and does not comply with the contract.</en>
            <cn>不是getter，不符合约定。</cn>
            <tw>不是getter，不符合約定。</tw>
        */
        fn width(&self) -> bool {
            self.width > 0
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}.", rect1.width);
    }
}

fn p126b() {
    println!("\nIn p126b()");

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        /*
            <en>It is a getter and conforms to the convention.</en>
            <cn>是getter，符合约定。</cn>
            <tw>是getter，符合約定。</tw>
        */
        fn width(&self) -> u32 {
            self.width
        }

        /*
            <en>It is a setter and conforms to the agreement.</en>
            <cn>是setter，符合约定。</cn>
            <tw>是setter，符合約定。</tw>
        */
        fn set_width(&mut self, width: u32) {
            self.width = width;
        }

        fn height(&self) -> u32 {
            self.height
        }

        fn set_height(&mut self, height: u32) {
            self.height = height;
        }
    }

    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let area = rect1.area();
    println!(
        "Width: {}, height: {}, area is {}.",
        rect1.width(),
        rect1.height(),
        area
    );
    println!(
        "Width: {}, height: {}, area is {}.",
        rect1.width, rect1.height, area
    );

    rect1.set_width(40);
    rect1.set_height(55);
    println!("After changing width and height:");
    let area = rect1.area();
    println!(
        "Width: {}, height: {}, area is {}.",
        rect1.width(),
        rect1.height(),
        area
    );
    println!(
        "Width: {}, height: {}, area is {}.",
        rect1.width, rect1.height, area
    );
}

fn p126c() {
    println!("\nIn p126c()");

    mod chapter07 {
        #[derive(Debug)]
        #[allow(dead_code)]
        pub struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            pub fn new(width: u32, height: u32) -> Self {
                Self { width, height }
            }

            pub fn area(&self) -> u32 {
                self.width * self.height
            }

            pub fn width(&self) -> u32 {
                self.width
            }

            pub fn set_width(&mut self, width: u32) {
                self.width = width;
            }

            pub fn height(&self) -> u32 {
                self.height
            }

            pub fn set_height(&mut self, height: u32) {
                self.height = height;
            }
        }
    }

    use chapter07::Rectangle;

    // error
    // let mut _rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };
    /*
        error[E0451]: field `width` of struct `chapter07::Rectangle` is private
        --> src\main.rs:578:9
            |
        578 |         width: 30,
            |         ^^^^^^^^^ private field

        error[E0451]: field `height` of struct `chapter07::Rectangle` is private
        --> src\main.rs:579:9
            |
        579 |         height: 50,
            |         ^^^^^^^^^^ private field

        For more information about this error, try `rustc --explain E0451`.
    */

    let mut rect1 = Rectangle::new(30, 50);

    let area = rect1.area();
    println!(
        "Width: {}, height: {}, area is {}.",
        rect1.width(),
        rect1.height(),
        area
    );

    rect1.set_width(40);
    rect1.set_height(55);
    println!("After changing width and height:");
    let area = rect1.area();
    println!(
        "Width: {}, height: {}, area is {}.",
        rect1.width(),
        rect1.height(),
        area
    );

    // error
    // println!(
    //     "Width: {}, height: {}, area is {}.",
    //     rect1.width, rect1.height, area
    // );
    /*
        error[E0616]: field `width` of struct `chapter07::Rectangle` is private
        --> src\main.rs:621:15
            |
        621 |         rect1.width, rect1.height, area
            |               ^^^^^ private field
            |
        help: a method `width` also exists, call it with parentheses
            |
        621 |         rect1.width(), rect1.height, area
            |                    ++

        error[E0616]: field `height` of struct `chapter07::Rectangle` is private
        --> src\main.rs:621:28
            |
        621 |         rect1.width, rect1.height, area
            |                            ^^^^^^ private field
            |
        help: a method `height` also exists, call it with parentheses
            |
        621 |         rect1.width, rect1.height(), area
            |                                  ++

        For more information about this error, try `rustc --explain E0616`.
    */
}

fn p127a() {
    println!("\nIn p127a()");

    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        fn distance(&self, other: &Point) -> f64 {
            let x_squared = f64::powi(other.x - self.x, 2);
            let y_squared = f64::powi(other.y - self.y, 2);

            f64::sqrt(x_squared + y_squared)
        }
    }

    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 5.0, y: 6.5 };
    println!("p1.distance(&p2) is {}", p1.distance(&p2));
    println!("(&p1).distance(&p2) is {}", (&p1).distance(&p2));
}

// Skipped p127b.

fn p128a() {
    println!("\nIn p128a()");

    #[derive(Debug, Copy, Clone)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Self) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rect1: {rect1:?}\nrect2: {rect2:?}\nrect2: {rect3:?}");
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn p129a() {
    println!("\nIn p129a()");
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    let sq = Rectangle::square(3);
    println!("sq is {sq:?}");
}

fn p129b() {
    println!("\nIn p129b()");

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!(
        "The areas of rect1, rect2 and rect3 are {}, {} are {}.",
        rect1.area(),
        rect2.area(),
        rect3.area()
    );
}

use std::time::Instant;
#[allow(unused_variables)]
fn main() {
    let main_function_name = "chapter05_main";
    let start_chapter_line = "Chapter 05";
    done_and_show_used_milliseconds(main_function_name, || {
        println!("start: {:?}", Instant::now());
        let action = || {
            println!("{start_chapter_line}");

            p114a();
            p114b();

            p115a();
            p115b();

            p116a();

            p117a();

            p118a();
            p118b();

            p120a();

            p121a();
            p121b();

            p122a();

            p123a();

            p124a();

            p125a();

            p126a();
            p126b();
            p126c();

            p127a();

            p128a();

            p129a();
            p129b();
        };

        let functions = vec![
            ("p114a", p114a as fn()),
            ("p114b", p114b as fn()),
            ("p115a", p115a as fn()),
            ("p115b", p115b as fn()),
            ("p116a", p116a as fn()),
            ("p117a", p117a as fn()),
            ("p118a", p118a as fn()),
            ("p118b", p118b as fn()),
            ("p120a", p120a as fn()),
            ("p121a", p121a as fn()),
            ("p121b", p121b as fn()),
            ("p122a", p122a as fn()),
            ("p123a", p123a as fn()),
            ("p124a", p124a as fn()),
            ("p125a", p125a as fn()),
            ("p126a", p126a as fn()),
            ("p126b", p126b as fn()),
            ("p126c", p126c as fn()),
            ("p127a", p127a as fn()),
            ("p128a", p128a as fn()),
            ("p129a", p129a as fn()),
            ("p129b", p129b as fn()),
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
