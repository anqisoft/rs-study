// 137.rs
// https://google.github.io/comprehensive-rust/zh-CN/async/runtimes/tokio.html

// See: example_137\src\main.rs
// I don't know how to debug it directly. So I converted it to a new project and debugged it.

use tokio::time;

async fn count_to(count: i32) {
    for i in 1..=count {
        println!("Count in task: {i}!");
        time::sleep(time::Duration::from_millis(5)).await;
    }
}

#[tokio::main]
async fn main() {
    tokio::spawn(count_to(10));

    for i in 1..5 {
        println!("Main task: {i}");
        time::sleep(time::Duration::from_millis(5)).await;
    }
}

/* result:

*/
