use add_one;

// error[E0432]: unresolved import `rand`
// no external crate `rand`
// use rand;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}
