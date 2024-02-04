// 142.rs
// https://google.github.io/comprehensive-rust/zh-CN/async/pitfalls/blocking-executor.html

// See: example_142\src\main.rs
// I don't know how to debug it directly. So I converted it to a new project and debugged it.

use futures::future::join_all;
use std::time::Instant;

async fn sleep_ms(start: &Instant, id: u64, duration_ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(duration_ms));
    println!(
        "future {id} slept for {duration_ms}ms, finished after {}ms",
        start.elapsed().as_millis()
    );
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let start = Instant::now();
    let sleep_futures = (1..=10).map(|t| sleep_ms(&start, t, t * 10));
    join_all(sleep_futures).await;
}
/* result:

*/
