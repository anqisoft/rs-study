// 081.rs
// https://www.runoob.com/rust/rust-concurrency.html
// The wrong file is error/024.rs

use std::thread;

fn main() {
    let s = "hello";

    let handle = thread::spawn(move || {
        println!("{}", s);
    });

    handle.join().unwrap();

    another();
}

fn another() {
    println!("\nIn another()");
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

/* result:
hello

In another()
Got: hi
*/
