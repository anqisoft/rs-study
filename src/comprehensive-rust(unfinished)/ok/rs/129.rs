// 129.rs
// https://google.github.io/comprehensive-rust/zh-CN/concurrency/channels.html

use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    tx.send(10).unwrap();
    tx.send(20).unwrap();

    println!("Received: {:?}", rx.recv());
    println!("Received: {:?}", rx.recv());

    let tx2 = tx.clone();
    tx2.send(30).unwrap();
    println!("Received: {:?}", rx.recv());
}

/* result:
Received: Ok(10)
Received: Ok(20)
Received: Ok(30)
*/
