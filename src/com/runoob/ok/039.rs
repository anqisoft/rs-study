// 039.rs
// https://www.runoob.com/rust/rust-project-management.html

// main.rs
mod second_module;

fn main() {
    println!("This is the main module.");
    println!("{}", second_module::message());
}

/* result:
This is the main module.
This is the 2nd module.
*/