// 121.rs
// https://google.github.io/comprehensive-rust/zh-CN/unsafe/mutable-static-variables.html

static HELLO_WORLD: &str = "Hello, world!";

fn test1() {
    println!("In test1()");
    println!("HELLO_WORLD: {HELLO_WORLD}");
}

fn test2() {
    println!("\nIn test2()");

    static mut COUNTER: u32 = 0;
    fn add_to_counter(inc: u32) {
        unsafe { COUNTER += inc; }  // Potential data race!
    }

    add_to_counter(42);
    unsafe { println!("COUNTER: {COUNTER}"); }  // Potential data race!

    add_to_counter(18);
    unsafe { println!("COUNTER: {COUNTER}"); }  // Potential data race!
}

fn main() {
    test1();
    test2();
}

/* result:
In test1()
HELLO_WORLD: Hello, world!

In test2()
COUNTER: 42
COUNTER: 60
*/
