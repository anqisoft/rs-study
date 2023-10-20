// 034.rs
// https://www.runoob.com/rust/rust-enum.html

fn main() {
    a();
    b();
    c();
    d();
    e();
    f();
}

fn a() {
    println!("in a():");

    #[derive(Debug)]
    enum Book {
        Papery, Electronic
    }

    let book = Book::Papery;
    println!("{:?}", book);
}

fn b() {
    println!("\nin b():");

    #[derive(Debug)]
    enum Book {
        Papery(u32),
        Electronic(String),
    }

    let book = Book::Papery(1001);
    let ebook = Book::Electronic(String::from("url://..."));

    println!("book: {:?}", book);
    println!("ebook: {:?}", ebook);
}

fn c() {
    println!("\nin c():");

    #[derive(Debug)]
    enum Book {
        Papery { index: u32 },
        Electronic { url: String },
    }
    let book = Book::Papery{index: 1001};
    println!("book: {:?}", book);
}

fn d() {
    println!("\nin d():");

    enum Book {
        Papery {index: u32},
        Electronic {url: String},
    }

    let book = Book::Papery{index: 1001};
    let ebook = Book::Electronic{url: String::from("url...")};

    match book {
        Book::Papery { index } => {
            println!("Papery book {}", index);
        },
        Book::Electronic { url } => {
            println!("E-book {}", url);
        }
    }
}

fn e() {
    println!("\nin e():");

    enum Book {
        Papery(u32),
        Electronic { url: String },
    }
    let book = Book::Papery(1001);

    match book {
        Book::Papery(i) => {
            println!("{}", i);
        },
        Book::Electronic { url } => {
            println!("{}", url);
        }
    }
}

fn f() {
    println!("\nin f():");

    let t = "abc";
    match t {
        "abc" => println!("Yes"),
        _ => {},
    }
}

/* result:
in a():
Papery

in b():
book: Papery(1001)
ebook: Electronic("url://...")

in c():
book: Papery { index: 1001 }

in d():
Papery book 1001

in e():
1001

in f():
Yes
*/

/* warnings:
warning: unused variable: `ebook`
  --> ..\034.rs:64:9
   |
64 |     let ebook = Book::Electronic{url: String::from("url...")};
   |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_ebook`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: variant `Electronic` is never constructed
  --> ..\034.rs:20:17
   |
19 |     enum Book {
   |          ---- variant in this enum
20 |         Papery, Electronic
   |                 ^^^^^^^^^^
   |
   = note: `Book` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` on by default

warning: field `index` is never read
  --> ..\034.rs:48:18
   |
48 |         Papery { index: u32 },
   |         ------   ^^^^^
   |         |
   |         field in this variant

warning: variant `Electronic` is never constructed
  --> ..\034.rs:49:9
   |
47 |     enum Book {
   |          ---- variant in this enum
48 |         Papery { index: u32 },
49 |         Electronic { url: String },
   |         ^^^^^^^^^^
   |
   = note: `Book` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis

warning: variant `Electronic` is never constructed
  --> ..\034.rs:81:9
   |
79 |     enum Book {
   |          ---- variant in this enum
80 |         Papery(u32),
81 |         Electronic { url: String },
   |         ^^^^^^^^^^

warning: 5 warnings emitted
*/