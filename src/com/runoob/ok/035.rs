// 035.rs
// https://www.runoob.com/rust/rust-enum.html

/*
enum Option<T> {
    Some(T),
    None,
}
*/

fn main() {
    println!("Hello, Rustaceans! Say by 035.rs.");

    a();
    b();
    c();
    d();
    e();
    f();
}

fn a() {
    println!("In a():");

    let opt = Option::Some("Hello");
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }
}

fn b() {
    println!("\nIn b():");

    let opt: Option<&str> = Option::None;
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }
}

fn c() {
    println!("\nIn c():");

    let t = Some(64);
    match t {
        Some(64) => println!("Yes"),
        _ => println!("No"),
    }
}

fn d() {
    println!("\nIn d():");

    let i = 0;
    match i {
        0 => println!("zero"),
        _ => {},
    }
}

fn e() {
    println!("\nIn e():");

    let i = 0;
    if let 0 = i {
        println!("zero");
    }
}

fn f() {
    println!("\nIn f():");

    enum Book {
        Papery(u32),
        Electronic(String)
    }

    let book = Book::Electronic(String::from("url"));
    if let Book::Papery(index) = book {
        println!("Papery {}", index);
    } else {
        println!("Not papery book");
    }
}

/* result:
Hello, Rustaceans! Say by 035.rs.
In a():
Hello

In b():
opt is nothing

In c():
Yes

In d():
zero

In e():
zero

In f():
Not papery book
*/

/* warnings:
warning: variant `Papery` is never constructed
  --> ..\035.rs:83:9
   |
82 |     enum Book {
   |          ---- variant in this enum
83 |         Papery(u32),
   |         ^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted
*/