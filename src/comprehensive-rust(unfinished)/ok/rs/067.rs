// 067.rs
// https://google.github.io/comprehensive-rust/zh-CN/structs/field-shorthand.html

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Default for Person {
    // OK
    // fn default() -> Person {
    //     Person {
    //         name: "Bot".to_string(),
    //         age: 0,
    //     }
    // }

    // Person -> Self
    fn default() -> Self {
        Self {
            name: "Bot".to_string(),
            age: 0,
        }
    }
}

fn create_default() {
    let tmp = Person {
        ..Person::default()
    };
    println!("tmp: {tmp:#?}");

    let tmp = Person {
        name: "Sam".to_string(),
        ..Person::default()
    };
    println!("tmp: {tmp:#?}");
}

fn main() {
    create_default();
}

/* result:
tmp: Person {
    name: "Bot",
    age: 0,
}
tmp: Person {
    name: "Sam",
    age: 0,
}
*/
