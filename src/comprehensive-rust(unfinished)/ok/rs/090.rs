// 090.rs
// https://google.github.io/comprehensive-rust/zh-CN/traits.html

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

fn greet<P: Pet>(pet: &P) {
    println!("Oh you're a cutie! What's your name? {}", pet.talk());
}

fn main() {
    let captain_floof = Cat { lives: 9 };
    let fido = Dog { name: String::from("Fido"), age: 5 };

    greet(&captain_floof);
    greet(&fido);
}

/* result:
Oh you're a cutie! What's your name? Miau!
Oh you're a cutie! What's your name? Woof, my name is Fido!
*/

/*
warning: field `age` is never read
 --> ..\rs\090.rs:4:28
  |
4 | struct Dog { name: String, age: i8 }
  |        ---                 ^^^
  |        |
  |        field in this struct
  |
  = note: `#[warn(dead_code)]` on by default

warning: field `lives` is never read
 --> ..\rs\090.rs:5:14
  |
5 | struct Cat { lives: i8 } // No name needed, cats won't respond anyway.
  |        ---   ^^^^^
  |        |
  |        field in this struct

warning: 2 warnings emitted
*/