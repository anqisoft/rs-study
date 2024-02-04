// 081.rs
// https://google.github.io/comprehensive-rust/zh-CN/modules.html

mod foo {
    pub fn do_something() {
        println!("In the foo module");
    }
}

mod bar {
    pub fn do_something() {
        println!("In the bar module");
    }
}

fn main() {
    foo::do_something();
    bar::do_something();
}

/* result:
In the foo module
In the bar module
*/
