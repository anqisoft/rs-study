// 130.rs
// https://google.github.io/comprehensive-rust/zh-CN/concurrency/channels/unbounded.html

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 1..10 {
            tx.send(format!("Message {i}")).unwrap();
            println!("{thread_id:?}: sent Message {i}");
        }
        println!("{thread_id:?}: done");
    });
    thread::sleep(Duration::from_millis(100));

    for msg in rx.iter() {
        println!("Main: got {msg}");
    }
}

/* result:
ThreadId(2): sent Message 1
ThreadId(2): sent Message 2
ThreadId(2): sent Message 3
ThreadId(2): sent Message 4
ThreadId(2): sent Message 5
ThreadId(2): sent Message 6
ThreadId(2): sent Message 7
ThreadId(2): sent Message 8
ThreadId(2): sent Message 9
ThreadId(2): done
Main: got Message 1
Main: got Message 2
Main: got Message 3
Main: got Message 4
Main: got Message 5
Main: got Message 6
Main: got Message 7
Main: got Message 8
Main: got Message 9
*/
