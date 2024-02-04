/*
 * Copyright (c) 2023 anqisoft@gmail.com
 * chapter15\src\main.rs
 *
 * <en>https://doc.rust-lang.org/book/ch15-00-smart-pointers.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch15-00-smart-pointers.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch15-00-smart-pointers.html</tw>
*/
/* <en>
 * Created on Fri Dec 08 2023 18:13:15
 * Feature: Test the code of Chapter 15 in the Chinese version pdf.
 * The method name starts with p, followed by the PDF page number and the current page sub-number (a to z).
 * For example, p1a represents the first example on the first page of the PDF.
 * </en>
*/
/* <cn>
 * 创建：2023年12月8日 18:13:15
 * 功能：测试中文版pdf中第15章的代码。方法名以p开头，后面加pdf页序号及当页子序号（a到z），如p1a表示pdf第一页第一个例子。
 * </cn>
*/
/* <tw>
 * 創建：2023年12月8日 18:13:15
 * 功能：測試中文版pdf中第15章的程式碼。方法名稱以p開頭，後面加pdf頁序號及當頁子序號（a到z），如p1a表示pdf第一頁第一個例子。
 * </tw>
*/

/*
* <en>https://doc.rust-lang.org/book/ch15-01-box.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch15-01-box.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch15-01-box.html</tw>
*/

use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn p366a() {
    println!("\nIn p366a(), eg15-1");

    let b = Box::new(5);
    println!("b = {}", b);
}

fn p367a() {
    println!("\nIn p367a(), eg15-2");

    // #[allow(dead_code)]
    // enum List {
    //     Cons(i32, List),
    //     Nil,
    // }
}

fn p367b() {
    println!("\nIn p367b(), eg15-3, error: eg15-4");

    // #[allow(dead_code)]
    // #[derive(Debug)]
    // enum List {
    //     Cons(i32, List),
    //     Nil,
    // }

    // // Original:
    // // use crate::List::{Cons, Nil};
    // // Use in inner function
    // use List::{Cons, Nil};

    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    // println!("{:?}", &list);
}
/* eg15-4
  error[E0072]: recursive type `p367b::List` has infinite size
    --> src\main.rs:55:5
    |
  55 |     enum List {
    |     ^^^^^^^^^
  56 |         Cons(i32, List),
    |                   ---- recursive without indirection
    |
  help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
    |
  56 |         Cons(i32, Box<List>),
    |                   ++++    +

  For more information about this error, try `rustc --explain E0072`.
*/

// eg6-2
// enum Message {
//   Quit,
//   Move { x: i32, y: i32 },
//   Write(String),
//   ChangeColor(i32, i32, i32),
// }

fn p369a() {
    println!("\nIn p369a(), eg15-5");

    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    // Original:
    // use crate::List::{Cons, Nil};
    // Use in inner function
    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", &list);
}

/*
* <en>https://doc.rust-lang.org/book/ch15-02-deref.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch15-02-deref.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch15-02-deref.html</tw>
*/

fn p372a() {
    println!("\nIn p372a(), eg15-6");

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Don't use:
    // error
    // assert_eq!(5, y);
    /*
      error[E0277]: can't compare `{integer}` with `&{integer}`
        --> src\main.rs:131:5
          |
      131 |     assert_eq!(5, y);
          |     ^^^^^^^^^^^^^^^^ no implementation for `{integer} == &{integer}`
          |
          = help: the trait `PartialEq<&{integer}>` is not implemented for `{integer}`
          = help: the following other types implement trait `PartialEq<Rhs>`:
                    isize
                    i8
                    i16
                    i32
                    i64
                    i128
                    usize
                    u8
                  and 6 others
          = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
    */
}

fn p373a() {
    println!("\nIn p373a(), eg15-7");

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn p374a() {
    println!("\nIn p374a(), eg15-8, eg15-9");

    // eg15-8
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    // eg15-9
    let x = 5;
    #[allow(unused_variables)]
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // assert_eq!(5, *y);
    /*
      error[E0614]: type `p374a::MyBox<{integer}>` cannot be dereferenced
      --> src\main.rs:181:19
          |
      181 |     assert_eq!(5, *y);
    */
}

// eg15-8: eg8_to_10.rs

// eg15-10: eg8_to_10.rs

fn p374b() {
    println!("\nIn p374b(), eg15-10");

    // eg15-9
    let x = 5;
    let y = chapter15::eg08::MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // same to: assert_eq!(5, *(y.deref()));
}

// eg15-11
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn p376a() {
    println!("\nIn p376a(), eg15-11, eg15-12");

    // eg15-12
    let m = chapter15::eg08::MyBox::new(String::from("Rust"));
    hello(&m);
}

fn p377a() {
    println!("\nIn p377a(), eg15-11, eg15-13");

    let m = chapter15::eg08::MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
    // same to eg15-12
}

/*
* <en>https://doc.rust-lang.org/book/ch15-03-drop.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch15-03-drop.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch15-03-drop.html</tw>
*/

#[allow(unused_variables)]
fn p378a() {
    println!("\nIn p378a(), eg15-14");

    use chapter15::eg14::CustomSmartPointer;

    // fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    // }
}

#[allow(unused_variables)]
fn p379a() {
    println!("\nIn p379a(), eg15-15");

    use chapter15::eg14::CustomSmartPointer;
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    // c.drop();
    /*
      error[E0040]: explicit use of destructor method
        --> src\main.rs:258:7
          |
      258 |     c.drop();
          |       ^^^^ explicit destructor calls not allowed
          |
      help: consider using `drop` function
          |
      258 |     drop(c);
          |     +++++ ~

      For more information about this error, try `rustc --explain E0040`.
    */
    println!("CustomSmartPointer dropped before the end of main.");
}

fn p380a() {
    println!("\nIn p380a(), eg15-16");

    use chapter15::eg14::CustomSmartPointer;
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

/*
* <en>https://doc.rust-lang.org/book/ch15-04-rc.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch15-04-rc.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch15-04-rc.html</tw>
*/

#[allow(unused_variables)]
fn p383a() {
    println!("\nIn p383a(), eg15-17");

    // use chapter15::eg17::List::{Cons, Nil};

    // // fn main() {
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));
    // // }
}
/*
error[E0382]: use of moved value: `a`
   --> src\main.rs:305:30
    |
303 |     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    |         - move occurs because `a` has type `chapter15::eg17::List`, which does not implement the `Copy` trait
304 |     let b = Cons(3, Box::new(a));
    |                              - value moved here
305 |     let c = Cons(4, Box::new(a));
    |                              ^ value used here after move
*/

#[allow(unused_variables)]
fn p383b() {
    println!("\nIn p383b(), eg15-18");

    use chapter15::eg18::{Cons, Nil};

    // fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    // }
}

#[allow(unused_variables)]
fn p384a() {
    println!("\nIn p384a(), eg15-19");

    use chapter15::eg18::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

/*
* <en>https://doc.rust-lang.org/book/ch15-05-interior-mutability.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch15-05-interior-mutability.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch15-05-interior-mutability.html</tw>
*/
#[allow(unused_variables)]
fn p387a() {
    println!("\nIn p387a()");

    // let x = 5;
    // let y = &mut x;
}
/*
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
   --> src\main.rs:360:13
    |
360 |     let y = &mut x;
    |             ^^^^^^ cannot borrow as mutable
    |
help: consider changing this to be mutable
    |
359 |     let mut x = 5;
    |         +++
*/

fn p394a() {
    println!("\nIn p394a(), eg15-24");

    use chapter15::eg24::{Cons, Nil};
    // fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
    // }
}

/*
* <en>https://doc.rust-lang.org/book/ch15-06-reference-cycles.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch15-06-reference-cycles.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch15-06-reference-cycles.html</tw>
*/

fn p396a() {
    println!("\nIn p396a(), eg15-26");

    use chapter15::eg25::List::{Cons, Nil};

    // eg15-26
    // fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
    // }
}

fn p399a() {
    println!("\nIn p399a(), eg15-27");

    use chapter15::eg27::Node;

    // fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    #[allow(unused_variables)]
    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    // }
}

fn p401a() {
    println!("\nIn p401a(), eg15-28");

    use chapter15::eg28::Node;

    // fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // }
}

fn p402a() {
    println!("\nIn p402a(), eg15-29");

    use chapter15::eg28::Node;

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

use std::time::Instant;
#[allow(unused_variables)]
fn main() {
    let main_function_name = "chapter15_main";
    let start_chapter_line = "Chapter 15";
    done_and_show_used_milliseconds(main_function_name, || {
        println!("start: {:?}", Instant::now());
        let action = || {
            println!("{start_chapter_line}");

            p366a();
            p367a();
            p367b();
            p369a();
            p372a();
            p373a();
            p374a();
            p374b();
            p376a();
            p377a();
            p378a();
            p379a();
            p380a();
            p383a();
            p383b();
            p384a();
            p387a();
            p394a();
            p396a();
            p399a();
            p401a();
            p402a();
        };

        let functions = vec![
            ("p366a", p366a as fn()),
            ("p367a", p367a as fn()),
            ("p367b", p367b as fn()),
            ("p369a", p369a as fn()),
            ("p372a", p372a as fn()),
            ("p373a", p373a as fn()),
            ("p374a", p374a as fn()),
            ("p374b", p374b as fn()),
            ("p376a", p376a as fn()),
            ("p377a", p377a as fn()),
            ("p378a", p378a as fn()),
            ("p379a", p379a as fn()),
            ("p380a", p380a as fn()),
            ("p383a", p383a as fn()),
            ("p383b", p383b as fn()),
            ("p384a", p384a as fn()),
            ("p387a", p387a as fn()),
            ("p394a", p394a as fn()),
            ("p396a", p396a as fn()),
            ("p399a", p399a as fn()),
            ("p401a", p401a as fn()),
            ("p402a", p402a as fn()),
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
