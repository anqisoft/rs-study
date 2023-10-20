// 002.rs
// https://www.runoob.com/rust/rust-println.html

fn main() {
    let a = 12;
    println!("a is {}", a);

    // println!("a is {}, a again is {}", a, a);
    println!("a is {0}, a again is {0}", a); 

    println!("{{}}"); 
}

/* result:
a is 12
a is 12, a again is 12
{}
*/