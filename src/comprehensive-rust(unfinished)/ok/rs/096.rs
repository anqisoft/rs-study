// 096.rs
// https://google.github.io/comprehensive-rust/zh-CN/traits/iterator.html

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn main() {
    let fib = Fibonacci { curr: 0, next: 1 };
    for (i, n) in fib.enumerate().take(5) {
        println!("fib({i}): {n}");
    }
}

/* result:
fib(0): 1
fib(1): 1
fib(2): 2
fib(3): 3
fib(4): 5
*/
