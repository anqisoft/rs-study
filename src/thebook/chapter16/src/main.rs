/*
 * Copyright (c) 2023 anqisoft@gmail.com
 * chapter16\src\main.rs
 *
 * <en>https://doc.rust-lang.org/book/ch16-00-concurrency.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch16-00-concurrency.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch16-00-concurrency.html</tw>
*/

/* <en>
 * Created on Sat Dec 09 2023 11:21:42
 * Feature: Test the code of Chapter 16 in the Chinese version pdf.
 * The method name starts with p, followed by the PDF page number and the current page sub-number (a to z).
 * For example, p1a represents the first example on the first page of the PDF.
 * </en>
*/

/* <cn>
 * 创建：2023年12月9日 11:21:42
 * 功能：测试中文版pdf中第16章的代码。方法名以p开头，后面加pdf页序号及当页子序号（a到z），如p1a表示pdf第一页第一个例子。
 * </cn>
*/

/* <tw>
 * 創建：2023年12月9日 11:21:42
 *  功能：測試中文版pdf中第16章的程式碼。方法名稱以p開頭，後面加pdf頁序號及當頁子序號（a到z），如p1a表示pdf第一頁第一個例子。
 * </tw>
*/

/*
* <en>https://doc.rust-lang.org/book/ch16-01-threads.html </en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch16-01-threads.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch16-01-threads.html</tw>
*/
use std::thread;
use std::time::Duration;

fn p406a() {
    println!("\nIn p406a(), eg16-1");

    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn p407a() {
    println!("\nIn p407a(), eg16-2");

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn p408a() {
    println!("\nIn p408a()");

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn p409a() {
    println!("\nIn p409a(), eg16-3");

    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v);
    // });

    // handle.join().unwrap();
}
/*
error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
  --> src\main.rs:95:32
   |
95 |     let handle = thread::spawn(|| {
   |                                ^^ may outlive borrowed value `v`
96 |         println!("Here's a vector: {:?}", v);
   |                                           - `v` is borrowed here
   |
note: function requires argument type to outlive `'static`
  --> src\main.rs:95:18
   |
95 |       let handle = thread::spawn(|| {
   |  __________________^
96 | |         println!("Here's a vector: {:?}", v);
97 | |     });
   | |______^
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
   |
95 |     let handle = thread::spawn(move || {
   |                                ++++
*/

fn p410a() {
    println!("\nIn p410a(), eg16-4");

    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    drop(v); // oh no!

    handle.join().unwrap();
}
/*

*/

fn p410b() {
    println!("\nIn p410b(), eg16-5");

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

/*
* <en>https://doc.rust-lang.org/book/ch16-02-message-passing.html </en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch16-02-message-passing.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch16-02-message-passing.html</tw>
*/
use std::sync::mpsc;

fn p412a() {
    println!("\nIn p412a(), eg16-6");

    // let (tx, rx) = mpsc::channel();
}
/*
error[E0282]: type annotations needed
   --> src\main.rs:144:20
    |
144 |     let (tx, rx) = mpsc::channel();
    |                    ^^^^^^^^^^^^^ cannot infer type of the type parameter `T` declared on the function `channel`
    |
help: consider specifying the generic argument
    |
144 |     let (tx, rx) = mpsc::channel::<T>();
    |                                 +++++
*/

fn p413a() {
    println!("\nIn p413a(), eg16-7");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
}

fn p413b() {
    println!("\nIn p413b(), eg16-8");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn p414a() {
    println!("\nIn p414a(), eg16-9");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn p415a() {
    println!("\nIn p415a(), eg16-10");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn p415b() {
    println!("\nIn p415b(), eg16-11");

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

/*
* <en>https://doc.rust-lang.org/book/ch16-03-shared-state.html </en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch16-03-shared-state.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch16-03-shared-state.html</tw>
*/
// use std::sync::Mutex;
// use std::thread;
use std::sync::{Arc, Mutex};

#[allow(unused_imports)]
use std::rc::Rc;

fn p418a() {
    println!("\nIn p418a(), eg16-12");

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

fn p419a() {
    println!("\nIn p419a(), eg16-13");

    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn p420a() {
    println!("\nIn p420a(), eg16-14");

    // let counter = Rc::new(Mutex::new(0));
    // let mut handles = vec![];

    // for _ in 0..10 {
    //     let counter = Rc::clone(&counter);
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();

    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("Result: {}", *counter.lock().unwrap());
}
/*
error[E0277]: `Rc<Mutex<i32>>` cannot be sent between threads safely
   --> src\main.rs:315:36
    |
315 |           let handle = thread::spawn(move || {
    |                        ------------- ^------
    |                        |             |
    |  ______________________|_____________within this `{closure@src\main.rs:315:36: 315:43}`
    | |                      |
    | |                      required by a bound introduced by this call
316 | |             let mut num = counter.lock().unwrap();
317 | |
318 | |             *num += 1;
319 | |         });
    | |_________^ `Rc<Mutex<i32>>` cannot be sent between threads safely
    |
    = help: within `{closure@src\main.rs:315:36: 315:43}`, the trait `Send` is not implemented for `Rc<Mutex<i32>>`
note: required because it's used within this closure
   --> src\main.rs:315:36
    |
315 |         let handle = thread::spawn(move || {
    |                                    ^^^^^^^
note: required by a bound in `spawn`
   --> D:\anqi\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\std\src\thread\mod.rs:683:8
    |
680 | pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    |        ----- required by a bound in this function
...
683 |     F: Send + 'static,
    |        ^^^^ required by this bound in `spawn`
*/

fn p422a() {
    println!("\nIn p422a(), eg16-15");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

/*
* <en>https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html </en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch16-04-extensible-concurrency-sync-and-send.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch16-04-extensible-concurrency-sync-and-send.html</tw>
*/

fn main() {
    p406a();
    p407a();
    p408a();
    p409a();
    p410a();
    p410b();

    p412a();
    p413a();
    p413b();
    p414a();
    p415a();
    p415b();

    p418a();
    p419a();
    p420a();
    p422a();
}
