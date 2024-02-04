// 066.rs
// https://google.github.io/comprehensive-rust/zh-CN/structs/field-shorthand.html

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    // Ok
    // fn new(name: String, age: u8) -> Person {
    //     Person { name, age }
    // }
    
    // Person -> Self
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }
}

fn main() {
    let peter = Person::new(String::from("Peter"), 27);
    println!("{peter:?}");
}

/* result:
Person { name: "Peter", age: 27 }
*/

/*
warning: fields `name` and `age` are never read
 --> ..\rs\066.rs:6:5
  |
5 | struct Person {
  |        ------ fields in this struct
6 |     name: String,
  |     ^^^^
7 |     age: u8,
  |     ^^^
  |
  = note: `Person` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
  = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted
*/