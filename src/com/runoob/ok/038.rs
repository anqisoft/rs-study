// 038.rs
// https://www.runoob.com/rust/rust-project-management.html

mod some_module {
    pub enum Person {
        King {
            name: String
        },
        Queen
    }
}

fn main() {
    let person = some_module::Person::King{
        name: String::from("Blue")
    };
    match person {
        some_module::Person::King {name} => {
            println!("{}", name);
        }
        _ => {}
    }
}

/* result:
Blue
*/

/* warnings:
warning: variant `Queen` is never constructed
 --> ..\038.rs:9:9
  |
5 |     pub enum Person {
  |              ------ variant in this enum
...
9 |         Queen
  |         ^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted
*/