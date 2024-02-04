// 127.rs
// https://google.github.io/comprehensive-rust/zh-CN/concurrency/threads.html

use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Count in thread: {i}!");
            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 1..5 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(5));
    }
}

/* result:
Main thread: 1
Count in thread: 1!
Count in thread: 2!
Main thread: 2
Count in thread: 3!
Main thread: 3
Main thread: 4
Count in thread: 4!
Count in thread: 5!

Main thread: 1
Count in thread: 1!
Main thread: 2
Count in thread: 2!
Count in thread: 3!
Main thread: 3
Count in thread: 4!
Main thread: 4
Count in thread: 5!
*/
