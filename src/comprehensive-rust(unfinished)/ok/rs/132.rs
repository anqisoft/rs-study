// 132.rs
// https://google.github.io/comprehensive-rust/zh-CN/concurrency/shared_state/arc.html

use std::thread;
use std::sync::Arc;

fn main() {
    let v = Arc::new(vec![10, 20, 30]);
    let mut handles = Vec::new();
    for _ in 1..5 {
        let v = Arc::clone(&v);
        handles.push(thread::spawn(move || {
            let thread_id = thread::current().id();
            println!("{thread_id:?}: {v:?}");
        }));
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("v: {v:?}");
}

/* result:
ThreadId(2): [10, 20, 30]
ThreadId(3): [10, 20, 30]
ThreadId(5): [10, 20, 30]
ThreadId(4): [10, 20, 30]
v: [10, 20, 30]
*/
