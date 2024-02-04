// 135.rs
// https://google.github.io/comprehensive-rust/zh-CN/async/async-await.html

// See: example_135\src\main.rs
// I don't know how to debug it directly. So I converted it to a new project and debugged it.

use futures::executor::block_on;

async fn count_to(count: i32) {
    for i in 1..=count {
        println!("Count is: {i}!");
    }
}

async fn async_main(count: i32) {
    count_to(count).await;
}

fn main() {
    block_on(async_main(10));
}

/* result:
Count is: 1!
Count is: 2!
Count is: 3!
Count is: 4!
Count is: 5!
Count is: 6!
Count is: 7!
Count is: 8!
Count is: 9!
Count is: 10!
*/
