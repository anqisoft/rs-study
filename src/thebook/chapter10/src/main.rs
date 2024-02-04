/*
 * Copyright (c) 2023 anqisoft@gmail.com
 * chapter10\src\main.rs
 *
 * <en>https://doc.rust-lang.org/book/ch10-00-generics.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch10-00-generics.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch10-00-generics.html</tw>
 *
 * <en>
 * Created on Sat Dec 02 2023 19:57:34
 * Feature: Test the code of Chapter 10 in the Chinese version pdf.
 * The method name starts with p, followed by the PDF page number and the current page sub-number (a to z).
 * For example, p1a represents the first example on the first page of the PDF.
 * </en>
 *
 * <cn>
 * 创建：2023年12月2日 19:57:34
 * 功能：测试中文版pdf中第10章的代码。方法名以p开头，后面加pdf页序号及当页子序号（a到z），如p1a表示pdf第一页第一个例子。
 * </cn>
 *
 * <tw>
 * 創建：2023年12月2日 19:57:34
 * 功能：測試中文版pdf中第10章的程式碼。方法名稱以p開頭，後面加pdf頁序號及當頁子序號（a到z），如p1a表示pdf第一頁第一個例子。
 * </tw>
*/

/*
 * <en>https://doc.rust-lang.org/book/ch10-00-generics.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch10-00-generics.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch10-00-generics.html</tw>
*/

fn p207a() {
    println!("\nIn p207a(), eg10-1");

    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    assert_eq!(*largest, 100);
}

fn p208a() {
    println!("\nIn p208a(), eg10-2");

    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);
}

fn p208b() {
    println!("\nIn p208b(), eg10-3");

    // anqi
    // {
    //     fn largest(number_list: &Vec<i32>) -> i32 {
    //         let mut largest = number_list[0];
    //         for number in number_list {
    //             if *number > largest {
    //                 largest = *number;
    //             }
    //         }

    //         largest
    //     }

    //     let number_list = vec![34, 50, 25, 100, 65];
    //     println!("The largest number is {}", largest(&number_list));

    //     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    //     println!("The largest number is {}", largest(&number_list));
    // }

    fn largest(list: &[i32]) -> &i32 {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(*result, 100);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(*result, 6000);
}

/*
 * <en>https://doc.rust-lang.org/book/ch10-01-syntax.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch10-01-syntax.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch10-01-syntax.html</tw>
*/

fn p210a() {
    println!("\nIn p210a()");

    fn largest_i32(list: &[i32]) -> &i32 {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    fn largest_char(list: &[char]) -> &char {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(*result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(*result, 'y');
}

fn p211a() {
    println!("\nIn p211a(), eg10-5");

    // error
    // fn largest<T>(list: &[T]) -> &T {
    // ok
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
/*
error[E0369]: binary operation `>` cannot be applied to type `&T`
   --> src\main.rs:163:21
    |
163 |             if item > largest {
    |                ---- ^ ------- &T
    |                |
    |                &T
    |
help: consider restricting type parameter `T`
    |
158 |     fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
*/

fn p212a() {
    println!("\nIn p212a(), eg10-6");

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!(
        "integer is {integer:?}, and its type is {}",
        type_of(&integer)
    );
    println!("float is {float:?}, and its type is {}", type_of(&float));
}

fn p212b() {
    println!("\nIn p212b(), eg10-7");

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Point<T> {
        x: T,
        y: T,
    }

    // let wont_work = Point { x: 5, y: 4.0 };
    /*
        error[E0308]: mismatched types
        --> src\main.rs:221:38
            |
        221 |     let wont_work = Point { x: 5, y: 4.0 };
            |                                      ^^^ expected integer, found floating-point number

        For more information about this error, try `rustc --explain E0308`.
    */

    let _wont_work = Point {
        x: 5 as f64,
        y: 4.0,
    };
}

fn p213a() {
    println!("\nIn p213a(), eg10-8");

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Point<T, U> {
        x: T,
        y: U,
    }

    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
    println!(
        "both_integer is {both_integer:?}, and its type is {}",
        type_of(&both_integer)
    );
    println!(
        "both_float is {both_float:?}, and its type is {}",
        type_of(&both_float)
    );
    println!(
        "integer_and_float is {integer_and_float:?}, and its type is {}",
        type_of(&integer_and_float)
    );
}

fn p214a() {
    println!("\nIn p214a(), eg10-9");

    #[allow(dead_code)]
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}

fn p214b() {
    println!("\nIn p214b(), eg10-10");

    #[allow(dead_code)]
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p2: Point<f32> = Point { x: 3.0, y: 4.0 };
    println!("distance is {}", p2.distance_from_origin());

    // println!("distance is {}", p.distance_from_origin());
    /*
        error[E0599]: no method named `distance_from_origin` found for struct `p214b::Point<{integer}>` in the current scope
        --> src\main.rs:311:34
            |
        288 |     struct Point<T> {
            |     --------------- method `distance_from_origin` not found for this struct
        ...
        311 |     println!("distance is {}", p.distance_from_origin());
            |                                  ^^^^^^^^^^^^^^^^^^^^ method not found in `Point<{integer}>`
            |
            = note: the method was found for
                    - `p214b::Point<f32>`

        For more information about this error, try `rustc --explain E0599`.
    */
}

fn p215a() {
    println!("\nIn p215a(), eg10-11");

    #[derive(Debug)]
    struct Point<X1, Y1> {
        x: X1,
        y: Y1,
    }

    impl<X1, Y1> Point<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // println!("p1: {p1:?}, p2: {p2:?}, p3: {p3:?}");
    /*
        error[E0382]: borrow of moved value: `p1`
        --> src\main.rs:352:19
            |
        347 |     let p1 = Point { x: 5, y: 10.4 };
            |         -- move occurs because `p1` has type `p215a::Point<i32, f64>`, which does not implement the `Copy` trait
        348 |     let p2 = Point { x: "Hello", y: 'c' };
        349 |     let p3 = p1.mixup(p2);
            |                 --------- `p1` moved due to this method call
        ...
        352 |     println!("p1: {p1:?}, p2: {p2:?}, p3: {p3:?}");
            |                   ^^^^^^ value borrowed here after move
            |
        note: `p215a::Point::<X1, Y1>::mixup` takes ownership of the receiver `self`, which moves `p1`
        --> src\main.rs:339:26
            |
        339 |         fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            |                          ^^^^
            = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

        error[E0382]: borrow of moved value: `p2`
        --> src\main.rs:352:31
            |
        348 |     let p2 = Point { x: "Hello", y: 'c' };
            |         -- move occurs because `p2` has type `p215a::Point<&str, char>`, which does not implement the `Copy` trait
        349 |     let p3 = p1.mixup(p2);
            |                       -- value moved here
        ...
        352 |     println!("p1: {p1:?}, p2: {p2:?}, p3: {p3:?}");
            |                               ^^^^^^ value borrowed here after move
            |
        note: consider changing this parameter type in method `mixup` to borrow instead if owning the value isn't necessary
        --> src\main.rs:339:39
            |
        339 |         fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            |            ----- in this method       ^^^^^^^^^^^^^ this parameter takes ownership of the value
            = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

        For more information about this error, try `rustc --explain E0382`.
    */
}

#[allow(unused_variables)]
fn p216a() {
    println!("\nIn p216a()");

    #[allow(dead_code)]
    enum OptionI32 {
        Some(i32),
        None,
    }

    #[allow(dead_code)]
    enum OptionF64 {
        Some(f64),
        None,
    }

    let integer = OptionI32::Some(5);
    let float = OptionF64::Some(5.0);
}

/*
 * <en>https://doc.rust-lang.org/book/ch10-02-traits.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch10-02-traits.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch10-02-traits.html</tw>
*/

fn p217a() {
    println!("\nIn p218a(), eg10-12");

    pub trait Summary {
        fn summarize(&self) -> String;
    }
}

mod aggregator {
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
}
fn p218a() {
    println!("\nIn p218a(), eg10-13");

    // See: mod aggregator

    // anqi:
    use aggregator::{NewsArticle, Summary, Tweet};
    let article = NewsArticle {
        headline: "Announcing Rust 1.74.0".to_string(),
        location: "https://blog.rust-lang.org/2023/11/16/Rust-1.74.0.html".to_string(),
        author: "The Rust Release Team".to_string(),
        content: "The Rust team is happy to announce a new version of Rust, 1.74.0.\
Rust is a programming language empowering everyone to build reliable and efficient software.
If you have a previous version of Rust installed via rustup, you can get 1.74.0 with:
rustup update stable
"
        .to_string(),
    };
    println!("article.summarize():\n{}", article.summarize());

    let tweet = Tweet {
        username: "@rustlang".to_string(),
        content: "\
Rust 1.74.0 is now available! 🦀✨

This release comes with a new [lints] section in Cargo.toml, allows Self in `async fn` and `-> impl Trait` return types, adds the `Saturating` wrapper type to std, and more!

Check out the announcement and release notes:".to_string(),
        reply: false,
        retweet: false,
    };
    println!("tweet.summarize():\n{}", tweet.summarize());
}

fn p218b() {
    println!("\nIn p218b()");

    use aggregator::{Summary, Tweet};
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}

fn p219a() {
    println!("\nIn p219a(), eg10-14");

    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {}

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    // p220
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
        hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}

fn p220a() {
    println!("\nIn p220a()");

    pub trait Summary {
        fn summarize_author(&self) -> String;
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}

fn p221a() {
    println!("\nIn p221a()");

    use aggregator::{NewsArticle, Summary, Tweet};
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    let article = NewsArticle {
        headline: "Announcing Rust 1.74.0".to_string(),
        location: "https://blog.rust-lang.org/2023/11/16/Rust-1.74.0.html".to_string(),
        author: "The Rust Release Team".to_string(),
        content: "The Rust team is happy to announce a new version of Rust, 1.74.0.\
    Rust is a programming language empowering everyone to build reliable and efficient software.
    If you have a previous version of Rust installed via rustup, you can get 1.74.0 with:
    rustup update stable
    "
        .to_string(),
    };

    let tweet = Tweet {
            username: "@rustlang".to_string(),
            content: "\
    Rust 1.74.0 is now available! 🦀✨
    
    This release comes with a new [lints] section in Cargo.toml, allows Self in `async fn` and `-> impl Trait` return types, adds the `Saturating` wrapper type to std, and more!
    
    Check out the announcement and release notes:".to_string(),
            reply: false,
            retweet: false,
        };

    notify(&article);
    notify(&tweet);
}

fn p224a() {
    println!("\nIn p224a()");

    use aggregator::{Summary, Tweet};

    #[allow(dead_code)]
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}

fn p224b() {
    println!("\nIn p224b()");

    use aggregator::{NewsArticle, Summary, Tweet};

    // fn returns_summarizable(switch: bool) -> impl Summary {
    //     if switch {
    //         NewsArticle {
    //             headline: String::from("Penguins win the Stanley Cup Championship!"),
    //             location: String::from("Pittsburgh, PA, USA"),
    //             author: String::from("Iceburgh"),
    //             content: String::from(
    //                 "The Pittsburgh Penguins once again are the best \
    //     hockey team in the NHL.",
    //             ),
    //         }
    //     } else {
    //         Tweet {
    //             username: String::from("horse_ebooks"),
    //             content: String::from("of course, as you probably already know, people"),
    //             reply: false,
    //             retweet: false,
    //         }
    //     }
    // }
    /*
        error[E0308]: `if` and `else` have incompatible types
        --> src\main.rs:653:13
            |
        642 | /           if switch {
        643 | | /             NewsArticle {
        644 | | |                 headline: String::from("Penguins win the Stanley Cup Championship!"),
        645 | | |                 location: String::from("Pittsburgh, PA, USA"),
        646 | | |                 author: String::from("Iceburgh"),
        ...   | |
        650 | | |                 ),
        651 | | |             }
            | | |_____________- expected because of this
        652 | |           } else {
        653 | | /             Tweet {
        654 | | |                 username: String::from("horse_ebooks"),
        655 | | |                 content: String::from("of course, as you probably already know, people"),
        656 | | |                 reply: false,
        657 | | |                 retweet: false,
        658 | | |             }
            | | |_____________^ expected `NewsArticle`, found `Tweet`
        659 | |           }
            | |___________- `if` and `else` have incompatible types
            |
        help: you could change the return type to be a boxed trait object
            |
        641 |     fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
            |                                              ~~~~~~~        +
        help: if you change the return type to expect trait objects, box the returned expressions
            |
        643 ~             Box::new(NewsArticle {
        644 |                 headline: String::from("Penguins win the Stanley Cup Championship!"),
        ...
        650 |                 ),
        651 ~             })
        652 |         } else {
        653 ~             Box::new(Tweet {
        654 |                 username: String::from("horse_ebooks"),
        ...
        657 |                 retweet: false,
        658 ~             })
            |

        For more information about this error, try `rustc --explain E0308`.
    */

    fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
        if switch {
            Box::new(NewsArticle {
                headline: String::from("Penguins win the Stanley Cup Championship!"),
                location: String::from("Pittsburgh, PA, USA"),
                author: String::from("Iceburgh"),
                content: String::from(
                    "The Pittsburgh Penguins once again are the best \
        hockey team in the NHL.",
                ),
            })
        } else {
            Box::new(Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("of course, as you probably already know, people"),
                reply: false,
                retweet: false,
            })
        }
    }

    let summary1 = returns_summarizable(true);
    println!("The type of summary1 is {}", type_of(&summary1));

    let summary2 = returns_summarizable(false);
    println!("The type of summary2 is {}", type_of(&summary2));
}

fn p225a() {
    println!("\nIn p225a(), eg10-15");

    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    let integer_pair = Pair::new(5, 4);
    integer_pair.cmp_display();
    let integer_pair = Pair::new(4, 5);
    integer_pair.cmp_display();

    let string_pair = Pair::new("a", "b");
    string_pair.cmp_display();
    let string_pair = Pair::new("b", "a");
    string_pair.cmp_display();
}

/*
 * <en>https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch10-03-lifetime-syntax.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch10-03-lifetime-syntax.html</tw>
*/

fn p227a() {
    println!("\nIn p227a(), eg10-16");

    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r);
    /*
        error[E0597]: `x` does not live long enough
        --> src\main.rs:784:13
            |
        783 |         let x = 5;
            |             - binding `x` declared here
        784 |         r = &x;
            |             ^^ borrowed value does not live long enough
        785 |     }
            |     - `x` dropped here while still borrowed
        786 |     println!("r: {}", r);
            |                       - borrow later used here

        For more information about this error, try `rustc --explain E0597`.
    */
}

fn p228a() {
    println!("\nIn p228a(), eg10-17");

    // {
    //     let r;                 // ---------------+-- 'a
    //     {                      //                |
    //         let x = 5;         // -+-- 'b |      |
    //         r = &x;            //  |             |
    //     }                      // -+             |
    //     println!("r: {}", r);  //                |
    // }                          // ---------------+
}

fn p228b() {
    println!("\nIn p228b(), eg10-18");

    {
        let x = 5; // ----------+-- 'b
                   //                               |
        let r = &x; // --+-- 'a  |
                    //                       |       |
        println!("r: {}", r); // |       |
                              // ----------------------+       |
    } // ----------------------------+
}

fn p229a() {
    println!("\nIn p229a(), eg10-19");

    // let string1 = String::from("abcd");
    // let string2 = "xyz";
    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {}", result);

    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }
    /*
        error[E0106]: missing lifetime specifier
        --> src\main.rs:838:37
            |
        838 |     fn longest(x: &str, y: &str) -> &str {
            |                   ----     ----     ^ expected named lifetime parameter
            |
            = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
        help: consider introducing a named lifetime parameter
            |
        838 |     fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            |               ++++     ++          ++          ++

        For more information about this error, try `rustc --explain E0106`.
    */
}

fn p229b() {
    println!("\nIn p229b(), eg10-20");

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

fn p230a() {
    println!("\nIn p230a(), eg10-21");

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is '{}'", result);
    }

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}

fn p231a() {
    println!("\nIn p231a(), eg10-22");

    let string1 = String::from("abcd");
    let string2 = "efghijklmnopqrstuvwxyz";

    let result = first(string1.as_str(), string2);
    println!("The first string is {}", result);

    #[allow(unused_variables)]
    fn first<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }
}

fn p232a() {
    println!("\nIn p232a(), eg10-23");

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    // fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }
}
/*
error[E0597]: `string2` does not live long enough
   --> src\main.rs:920:44
    |
919 |         let string2 = String::from("xyz");
    |             ------- binding `string2` declared here
920 |         result = longest(string1.as_str(), string2.as_str());
    |                                            ^^^^^^^ borrowed value does not live long enough
921 |     }
    |     - `string2` dropped here while still borrowed
922 |     println!("The longest string is {}", result);
    |                                          ------ borrow later used here

For more information about this error, try `rustc --explain E0597`.
*/

fn p233a() {
    println!("\nIn p233a()");

    // let string1 = String::from("abcd");
    // let string2 = "efghijklmnopqrstuvwxyz";

    // let result = first(string1.as_str(), string2);
    // println!("The first string is {}", result);

    // #[allow(unused_variables)]
    // fn first<'a>(x: &str, y: &str) -> &'a str {
    //     let result = String::from("really long string");
    //     result.as_str()
    // }
}
/*
error[E0515]: cannot return value referencing local variable `result`
   --> src\main.rs:960:9
    |
960 |         result.as_str()
    |         ------^^^^^^^^^
    |         |
    |         returns a value referencing data owned by the current function
    |         `result` is borrowed here

For more information about this error, try `rustc --explain E0515`.
*/

fn p233b() {
    println!("\nIn p233b()");

    // let string1 = String::from("abcd");
    // let string2 = "xyz";

    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {}", result);

    // #[allow(unused_variables)]
    // fn longest<'a>(x: &str, y: &str) -> &'a str {
    //     let result = String::from("really long string");
    //     result.as_str()
    // }
}
/*
error[E0515]: cannot return value referencing local variable `result`
   --> src\main.rs:988:9
    |
988 |         result.as_str()
    |         ------^^^^^^^^^
    |         |
    |         returns a value referencing data owned by the current function
    |         `result` is borrowed here

For more information about this error, try `rustc --explain E0515`.
*/

fn p234a() {
    println!("\nIn p234a(), eg10-24");

    #[allow(dead_code)]
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("The result is {:?}", i);
}

#[allow(unused_variables)]
fn p236a() {
    println!("\nIn p236a(), eg10-25");

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    let my_string = String::from("hello world");
    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    println!("The first word is '{}'", word);
}

fn p237a() {
    println!("\nIn p237a()");

    #[allow(dead_code)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        #[allow(dead_code)]
        fn level(&self) -> i32 {
            3
        }
    }

    impl<'a> ImportantExcerpt<'a> {
        #[allow(dead_code)]
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let _i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn p238a() {
    println!("\nIn p238a()");

    use std::fmt::Display;
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result =
        longest_with_an_announcement(string1.as_str(), string2, "Today is someone's birthday!");
    println!("The longest string is {}", result);
}

use std::time::Instant;
#[allow(unused_variables)]
fn main() {
    let main_function_name = "chapter10_main";
    let start_chapter_line = "Chapter 10";
    done_and_show_used_milliseconds(main_function_name, || {
        println!("start: {:?}", Instant::now());
        let action = || {
            println!("{start_chapter_line}");

            p207a();
            p208a();
            p208b();
            p210a();
            p211a();
            p212a();
            p212b();
            p213a();
            p214a();
            p214b();
            p215a();
            p216a();
            p217a();
            p218a();
            p218b();
            p219a();
            p220a();
            p221a();
            p224a();
            p224b();
            p225a();
            p227a();
            p228a();
            p228b();
            p229a();
            p229b();
            p230a();
            p231a();
            p232a();
            p233a();
            p233b();
            p234a();
            p236a();
            p237a();
            p238a();
        };

        let functions = vec![
            ("p207a", p207a as fn()),
            ("p208a", p208a as fn()),
            ("p208b", p208b as fn()),
            ("p210a", p210a as fn()),
            ("p211a", p211a as fn()),
            ("p212a", p212a as fn()),
            ("p212b", p212b as fn()),
            ("p213a", p213a as fn()),
            ("p214a", p214a as fn()),
            ("p214b", p214b as fn()),
            ("p215a", p215a as fn()),
            ("p216a", p216a as fn()),
            ("p217a", p217a as fn()),
            ("p218a", p218a as fn()),
            ("p218b", p218b as fn()),
            ("p219a", p219a as fn()),
            ("p220a", p220a as fn()),
            ("p221a", p221a as fn()),
            ("p224a", p224a as fn()),
            ("p224b", p224b as fn()),
            ("p225a", p225a as fn()),
            ("p227a", p227a as fn()),
            ("p228a", p228a as fn()),
            ("p228b", p228b as fn()),
            ("p229a", p229a as fn()),
            ("p229b", p229b as fn()),
            ("p230a", p230a as fn()),
            ("p231a", p231a as fn()),
            ("p232a", p232a as fn()),
            ("p233a", p233a as fn()),
            ("p233b", p233b as fn()),
            ("p234a", p234a as fn()),
            ("p236a", p236a as fn()),
            ("p237a", p237a as fn()),
            ("p238a", p238a as fn()),
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
