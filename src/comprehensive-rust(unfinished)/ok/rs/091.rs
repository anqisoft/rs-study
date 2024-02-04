// 091.rs
// https://google.github.io/comprehensive-rust/zh-CN/traits/trait-objects.html

struct Dog { name: String, age: i8 }
struct Cat { lives: i8 } // No name needed, cats won't respond anyway.

trait Pet {
    fn talk(&self) -> String;
}

impl Pet for Dog {
    fn talk(&self) -> String { format!("Woof, my name is {}!", self.name) }
}

impl Pet for Cat {
    fn talk(&self) -> String { String::from("Miau!") }
}

fn main() {
    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Cat { lives: 9 }),
        Box::new(Dog { name: String::from("Fido"), age: 5 }),
    ];
    for pet in pets {
        println!("Hello, who are you? {}", pet.talk());
    }

    println!("{} {}", std::mem::size_of::<Dog>(), std::mem::size_of::<Cat>());
    println!("{} {}", std::mem::size_of::<&Dog>(), std::mem::size_of::<&Cat>());
    println!("{}", std::mem::size_of::<&dyn Pet>());
    println!("{}", std::mem::size_of::<Box<dyn Pet>>());
}

/* result:
Hello, who are you? Miau!
Hello, who are you? Woof, my name is Fido!
32 1
8 8
16
16
*/

/*
warning: field `age` is never read
 --> ..\rs\091.rs:4:28
  |
4 | struct Dog { name: String, age: i8 }
  |        ---                 ^^^
  |        |
  |        field in this struct
  |
  = note: `#[warn(dead_code)]` on by default

warning: field `lives` is never read
 --> ..\rs\091.rs:5:14
  |
5 | struct Cat { lives: i8 } // No name needed, cats won't respond anyway.
  |        ---   ^^^^^
  |        |
  |        field in this struct

warning: 2 warnings emitted
*/
