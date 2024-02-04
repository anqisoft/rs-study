// 078.rs
// https://google.github.io/comprehensive-rust/zh-CN/std/box-niche.html

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn main() {
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{list:?}");
}

/* result:
Cons(1, Cons(2, Nil))
*/
