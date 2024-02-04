// 068.rs
// https://google.github.io/comprehensive-rust/zh-CN/methods.html
// https://google.github.io/comprehensive-rust/zh-CN/methods/receiver.html

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn say_hello(&self) {
        // self` is a `&` reference, so the data it refers to cannot be written
        // help: consider changing this to be a mutable reference
        // fn say_hello(&mut self) {
        // self.name = self.name.clone() + " test";

        println!("Hello, my name is {}", self.name);
    }

    fn say_hello2(self) {
        // cannot assign
        // help: consider changing this to be mutable
        // fn say_hello2(mut self) {
        // self.name = self.name.clone() + " test";

        println!("2. Hello, my name is {}", self.name);
    }

    fn say_hello3(&mut self) {
        // `self.name` moved due to usage in operator
        //  move occurs because `self.name` has type `String`, which does not implement the `Copy` trait
        // note: calling this operator moves the left-hand side
        // self.name = self.name + " new";

        // Ok
        // self.name = self.name.clone() + " new";

        // Ok, the same
        self.name += " new";
        
        println!("3. Hello, my name is {}", self.name);
    }

    fn say_hello4(mut self) {
        // Ok
        // self.name = self.name.clone() + " new2";

        // Ok
        self.name += " new2";

        println!("4. Hello, my name is {}", self.name);
    }
}

fn main() {
    let peter = Person {
        name: String::from("Peter"),
        age: 27,
    };

    // Can be executed multiple times.
    peter.say_hello();
    peter.say_hello();

    // Only executed once.
    peter.say_hello2();
    // value used here after move
    // peter.say_hello2();

    let mut peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    // Can be executed multiple times.
    peter.say_hello3();
    peter.say_hello3();

    // Only executed once.
    peter.say_hello4();
    // value used here after move
    // peter.say_hello4();
}

/* result:
Hello, my name is Peter
Hello, my name is Peter
2. Hello, my name is Peter
3. Hello, my name is Peter new
3. Hello, my name is Peter new new
4. Hello, my name is Peter new new new2
*/

/*
warning: field `age` is never read
 --> ..\rs\068.rs:7:5
  |
5 | struct Person {
  |        ------ field in this struct
6 |     name: String,
7 |     age: u8,
  |     ^^^
  |
  = note: `Person` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
  = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted
*/