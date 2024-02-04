/*
 * Copyright (c) 2023 anqisoft@gmail.com
 * chapter20\src\main.rs
 *
 * <en>https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch20-00-final-project-a-web-server.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch20-00-final-project-a-web-server.html</tw>
*/

/* <en>
 * Created on Wed Dec 13 2023 10:18:09
 * Feature: Test the code of Chapter 20 in the Chinese version pdf.
 * The method name starts with p, followed by the PDF page number and the current page sub-number (a to z).
 * For example, p1a represents the first example on the first page of the PDF.
 * </en>
*/

/* <cn>
 * 创建：2023年12月13日 10:18:09
 * 功能：测试中文版pdf中第20章的代码。方法名以p开头，后面加pdf页序号及当页子序号（a到z），如p1a表示pdf第一页第一个例子。
 * </cn>
*/

/* <tw>
 * 創建：2023年12月13日 10:18:09
 *  功能：測試中文版pdf中第20章的程式碼。方法名稱以p開頭，後面加pdf頁序號及當頁子序號（a到z），如p1a表示pdf第一頁第一個例子。
 * </tw>
*/

#[allow(dead_code)]
const IP_AND_PORT: &str = "127.0.0.1:7878";

/*
* <en>https://doc.rust-lang.org/book/ch20-01-single-threaded.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch20-01-single-threaded.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch20-01-single-threaded.html</tw>
*/

#[allow(unused_variables)]
#[allow(dead_code)]
fn f01() {
    // eg20-1
    use std::net::TcpListener;

    // let listener = TcpListener::bind(IP_AND_PORT).unwrap();
    let listener = TcpListener::bind("127.0.0.1:7879").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
/* command-line window:
Connection established!
Connection established!
Connection established!
Connection established!
Connection established!
Connection established!
Connection established!
*/
/* navigator:
This site can’t be reachedThe connection was reset.
Try:

Checking the connection
Checking the proxy and the firewall
Running Windows Network Diagnostics
ERR_CONNECTION_RESET
*/

#[allow(dead_code)]
fn f02() {
    // eg20-2

    use std::{
        io::{prelude::*, BufReader},
        net::{TcpListener, TcpStream},
    };

    // fn main() {
    // let listener = TcpListener::bind(IP_AND_PORT).unwrap();
    let listener = TcpListener::bind("127.0.0.1:7880").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
    // }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        println!("Request: {:#?}", http_request);
    }
}
// /* command-line window:
// Request: [
//     "GET / HTTP/1.1",
//     "Host: 127.0.0.1:7880",
//     "Connection: keep-alive",
//     "Cache-Control: max-age=0",
//     "sec-ch-ua: \"Not_A Brand\";v=\"8\", \"Chromium\";v=\"120\", \"Google Chrome\";v=\"120\"",
//     "sec-ch-ua-mobile: ?0",
//     "sec-ch-ua-platform: \"Windows\"",
//     "Upgrade-Insecure-Requests: 1",
//     "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
//     "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
//     "Sec-Fetch-Site: none",
//     "Sec-Fetch-Mode: navigate",
//     "Sec-Fetch-User: ?1",
//     "Sec-Fetch-Dest: document",
//     "Accept-Encoding: gzip, deflate, br",
//     "Accept-Language: zh-CN,zh;q=0.9",
// ]
// */
/* navigator:  http://127.0.0.1:7880 (any page)
This page isn’t working127.0.0.1 didn’t send any data.
ERR_EMPTY_RESPONSE
*/

#[allow(unused_variables)]
#[allow(dead_code)]
fn f03() {
    // eg20-3

    use std::{
        io::{prelude::*, BufReader},
        net::{TcpListener, TcpStream},
    };

    // fn main() {
    // let listener = TcpListener::bind(IP_AND_PORT).unwrap();
    let listener = TcpListener::bind("127.0.0.1:7881").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
    // }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        // remove:
        // println!("Request: {:#?}", http_request);

        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    }
}
/* navigator:  http://127.0.0.1:7881 (any page) (blank page)
*/

// eg20-4: hello.html

#[allow(unused_variables)]
#[allow(dead_code)]
fn f04() {
    // eg20-5

    use std::{
        fs,
        io::{prelude::*, BufReader},
        net::{TcpListener, TcpStream},
    };

    // fn main() {
    // let listener = TcpListener::bind(IP_AND_PORT).unwrap();
    let listener = TcpListener::bind("127.0.0.1:7882").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
    // }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        // remove: eg20-2
        // println!("Request: {:#?}", http_request);

        // remove: eg20-3
        // let response = "HTTP/1.1 200 OK\r\n\r\n";

        // add: eg20-5
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}
/* navigator: http://127.0.0.1:7882 (any page)
Hello!
Hi from Rust
*/

#[allow(unused_variables)]
#[allow(dead_code)]
fn f05() {
    // eg20-6

    use std::{
        fs,
        io::{prelude::*, BufReader},
        net::{TcpListener, TcpStream},
    };

    // fn main() {
    // let listener = TcpListener::bind(IP_AND_PORT).unwrap();
    let listener = TcpListener::bind("127.0.0.1:7883").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
    // }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);

        // -- eg20-6 remove, start
        // let http_request: Vec<_> = buf_reader
        //     .lines()
        //     .map(|result| result.unwrap())
        //     .take_while(|line| !line.is_empty())
        //     .collect();

        // // remove: eg20-2
        // // println!("Request: {:#?}", http_request);

        // // remove: eg20-3
        // // let response = "HTTP/1.1 200 OK\r\n\r\n";

        // // add: eg20-5
        // let status_line = "HTTP/1.1 200 OK";
        // let contents = fs::read_to_string("hello.html").unwrap();
        // let length = contents.len();
        // let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        // stream.write_all(response.as_bytes()).unwrap();
        // -- eg20-6 remove, end

        // eg20-6: +
        let request_line = buf_reader.lines().next().unwrap().unwrap();

        if request_line == "GET / HTTP/1.1" {
            let status_line = "HTTP/1.1 200 OK";
            let contents = fs::read_to_string("hello.html").unwrap();
            let length = contents.len();

            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap();
        } else {
            // some other request
        }
    }
}
/* navigator: http://127.0.0.1:7883
Hello!
Hi from Rust
*/
/* navigator: http://127.0.0.1:7883/go
This page isn’t working127.0.0.1 didn’t send any data.
ERR_EMPTY_RESPONSE
*/

#[allow(unused_variables)]
#[allow(dead_code)]
fn f06() {
    // eg20-7

    use std::{
        fs,
        io::{prelude::*, BufReader},
        net::{TcpListener, TcpStream},
    };

    // fn main() {
    // let listener = TcpListener::bind(IP_AND_PORT).unwrap();
    let listener = TcpListener::bind("127.0.0.1:7884").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
    // }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let request_line = buf_reader.lines().next().unwrap().unwrap();

        if request_line == "GET / HTTP/1.1" {
            let status_line = "HTTP/1.1 200 OK";
            let contents = fs::read_to_string("hello.html").unwrap();
            let length = contents.len();

            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap();
        } else {
            // some other request
            // eg20-7: +
            let status_line = "HTTP/1.1 404 NOT FOUND";
            let contents = fs::read_to_string("404.html").unwrap();
            let length = contents.len();

            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}
/* navigator: http://127.0.0.1:7884
Hello!
Hi from Rust
*/
/* navigator: http://127.0.0.1:7884/any
Oops!
Sorry, I don't know what you're asking for.
*/

// eg20-8: 404.html

#[allow(unused_variables)]
#[allow(dead_code)]
fn f07() {
    // eg20-9

    use std::{
        fs,
        io::{prelude::*, BufReader},
        net::{TcpListener, TcpStream},
    };

    // fn main() {
    // let listener = TcpListener::bind(IP_AND_PORT).unwrap();
    let listener = TcpListener::bind("127.0.0.1:7885").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
    // }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let request_line = buf_reader.lines().next().unwrap().unwrap();

        // -- eg20-9 remove, start
        // if request_line == "GET / HTTP/1.1" {
        //     let status_line = "HTTP/1.1 200 OK";
        //     let contents = fs::read_to_string("hello.html").unwrap();
        //     let length = contents.len();

        //     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        //     stream.write_all(response.as_bytes()).unwrap();
        // } else {
        //     // some other request
        //     // eg20-7: +
        //     let status_line = "HTTP/1.1 404 NOT FOUND";
        //     let contents = fs::read_to_string("404.html").unwrap();
        //     let length = contents.len();

        //     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        //     stream.write_all(response.as_bytes()).unwrap();
        // }
        // -- eg20-9 remove, end

        // eg20-9 +
        let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

        let contents = fs::read_to_string(filename).unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}
/* navigator: http://127.0.0.1:7885
Hello!
Hi from Rust
*/
/* navigator: http://127.0.0.1:7885/any
Oops!
Sorry, I don't know what you're asking for.
*/

/*
* <en>https://doc.rust-lang.org/book/ch20-02-multithreaded.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch20-02-multithreaded.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch20-02-multithreaded.html</tw>
*/

#[allow(unused_variables)]
#[allow(dead_code)]
fn f08() {
    // eg20-10

    use std::{
        fs,
        io::{prelude::*, BufReader},
        net::{TcpListener, TcpStream},
        thread,
        time::Duration,
    };

    // fn main() {
    // let listener = TcpListener::bind(IP_AND_PORT).unwrap();
    let listener = TcpListener::bind("127.0.0.1:7886").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
    // }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let request_line = buf_reader.lines().next().unwrap().unwrap();

        // eg20-10 remove, start
        // let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        //     ("HTTP/1.1 200 OK", "hello.html")
        // } else {
        //     ("HTTP/1.1 404 NOT FOUND", "404.html")
        // };
        // -- eg20-10 remove, end

        // eg20-10 +
        let (status_line, filename) = match &request_line[..] {
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
            "GET /sleep HTTP/1.1" => {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", "hello.html")
            }
            _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
        };

        let contents = fs::read_to_string(filename).unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}
/* navigator: http://127.0.0.1:7886
Hello!
Hi from Rust
*/
/* navigator: http://127.0.0.1:7886/sleep (all slow)
Hello!
Hi from Rust
*/
/* navigator: http://127.0.0.1:7886/any
Oops!
Sorry, I don't know what you're asking for.
*/

#[allow(unused_variables)]
#[allow(dead_code)]
fn f09() {
    // eg20-11

    use std::{
        fs,
        io::{prelude::*, BufReader},
        net::{TcpListener, TcpStream},
        thread,
        time::Duration,
    };

    // fn main() {
    // let listener = TcpListener::bind(IP_AND_PORT).unwrap();
    let listener = TcpListener::bind("127.0.0.1:7887").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // eg20-11, remove
        // handle_connection(stream);

        // eg20-11, +
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
    // }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let request_line = buf_reader.lines().next().unwrap().unwrap();

        let (status_line, filename) = match &request_line[..] {
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
            "GET /sleep HTTP/1.1" => {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", "hello.html")
            }
            _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
        };

        let contents = fs::read_to_string(filename).unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}
/* navigator: http://127.0.0.1:7887
Hello!
Hi from Rust
*/
/* navigator: http://127.0.0.1:7887/sleep (slow only this page)
Hello!
Hi from Rust
*/
/* navigator: http://127.0.0.1:7887/any
Oops!
Sorry, I don't know what you're asking for.
*/

fn handle_connection(mut stream: std::net::TcpStream) {
    use std::{
        fs,
        io::{prelude::*, BufReader},
        // net::{TcpListener, TcpStream},
        thread,
        time::Duration,
    };

    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
#[allow(unused_variables)]
#[allow(dead_code)]
/* fn f10() {
    // eg20-12
    use std::net::TcpListener;

    // fn main() {
    // let listener = TcpListener::bind(IP_AND_PORT).unwrap();
    let listener = TcpListener::bind("127.0.0.1:7888").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
} */
/*
error[E0433]: failed to resolve: use of undeclared type `ThreadPool`
   --> src\main.rs:568:16
    |
568 |     let pool = ThreadPool::new(4);
    |                ^^^^^^^^^^ use of undeclared type `ThreadPool`

error[E0425]: cannot find function `handle_connection` in this scope
   --> src\main.rs:574:13
    |
574 |             handle_connection(stream);
    |             ^^^^^^^^^^^^^^^^^ not found in this scope

Some errors have detailed explanations: E0425, E0433.
*/

/* fn f10() {
    // eg20-12
    use hello::ThreadPool;
    use std::net::TcpListener;

    // fn main() {
    // let listener = TcpListener::bind(IP_AND_PORT).unwrap();
    let listener = TcpListener::bind("127.0.0.1:7888").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    mod hello {
        pub struct ThreadPool;
    }
} */
/*
error[E0599]: no function or associated item named `new` found for struct `ThreadPool` in the current scope
   --> src\main.rs:601:28
    |
601 |     let pool = ThreadPool::new(4);
    |                            ^^^ function or associated item not found in `ThreadPool`
...
612 |         pub struct ThreadPool;
    |         --------------------- function or associated item `new` not found for this struct

error[E0425]: cannot find function `handle_connection` in this scope
   --> src\main.rs:607:13
    |
607 |             handle_connection(stream);
    |             ^^^^^^^^^^^^^^^^^ not found in this scope

Some errors have detailed explanations: E0425, E0599.
*/

/* fn f10() {
    // eg20-12
    use hello::ThreadPool;
    use std::net::TcpListener;

    // fn main() {
    // let listener = TcpListener::bind(IP_AND_PORT).unwrap();
    let listener = TcpListener::bind("127.0.0.1:7888").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    mod hello {
        pub struct ThreadPool;

        impl ThreadPool {
            pub fn new(size: usize) -> ThreadPool {
                ThreadPool
            }
        }
    }
} */
/*
error[E0599]: no method named `execute` found for struct `ThreadPool` in the current scope
   --> src\main.rs:646:14
    |
646 |         pool.execute(|| {
    |         -----^^^^^^^ method not found in `ThreadPool`
...
652 |         pub struct ThreadPool;
    |         --------------------- method `execute` not found for this struct

error[E0425]: cannot find function `handle_connection` in this scope
   --> src\main.rs:647:13
    |
647 |             handle_connection(stream);
    |             ^^^^^^^^^^^^^^^^^ not found in this scope

Some errors have detailed explanations: E0425, E0599.
*/

/* fn f10() {
    // eg20-12
    use hello::ThreadPool;
    use std::net::TcpListener;

    // fn main() {
    // let listener = TcpListener::bind(IP_AND_PORT).unwrap();
    let listener = TcpListener::bind("127.0.0.1:7888").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    mod hello {
        pub struct ThreadPool;

        impl ThreadPool {
            pub fn new(size: usize) -> ThreadPool {
                ThreadPool
            }

            pub fn execute<F>(&self, f: F)
            where
                F: FnOnce() + Send + 'static,
            {
            }
        }
    }
} */
/* compile ok */
/* navigator: http://127.0.0.1:7888
This site can’t be reachedThe webpage at http://127.0.0.1:7888/ might be temporarily down or it may have moved permanently to a new web address.
ERR_SOCKET_NOT_CONNECTED
*/

/* eg20-12 to eg20-15
fn f10() {
    // eg20-12
    use hello::ThreadPool;
    use std::net::TcpListener;

    // let listener = TcpListener::bind(IP_AND_PORT).unwrap();
    let listener = TcpListener::bind("127.0.0.1:7888").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    mod hello {
        // eg20-14 remove
        // pub struct ThreadPool;

        // eg20-14 +
        use std::thread;

        // eg20-15, remove, start
        // pub struct ThreadPool {
        //     threads: Vec<thread::JoinHandle<()>>,
        // }
        // eg20-15, remove, start

        // eg20-15, +
        // 2. Change ThreadPool to hold a vector of Worker instances.
        pub struct ThreadPool {
            // threads: Vec<Worker>,
            workers: Vec<Worker>,
        }

        impl ThreadPool {
            // eg20-13 remove, start
            // pub fn new(size: usize) -> ThreadPool {
            //     ThreadPool
            // }
            // eg20-13 remove, end

            // eg20-14 remove, start
            // // eg20-13 +
            // /// Create a new ThreadPool.
            // ///
            // /// The size is the number of threads in the pool.
            // ///
            // /// # Panics
            // ///
            // /// The `new` function will panic if the size is zero.
            // pub fn new(size: usize) -> ThreadPool {
            //     assert!(size > 0);

            //     ThreadPool
            // }
            // // TODO
            // // pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> { }
            // eg20-14 remove, end

            // eg20-15, remove, start
            // // eg20-14 +
            // pub fn new(size: usize) -> ThreadPool {
            //     assert!(size > 0);

            //     let mut threads = Vec::with_capacity(size);

            //     for _ in 0..size {
            //         // create some threads and store them in the vector
            //     }

            //     ThreadPool { threads }
            // }
            // eg20-15, remove, end

            // eg20-15 +
            // 4. In ThreadPool::new, use the for loop counter to generate an id,
            //    create a new Worker with that id, and store the worker in the vector.
            pub fn new(size: usize) -> ThreadPool {
                assert!(size > 0);

                // let mut threads = Vec::with_capacity(size);
                let mut workers = Vec::with_capacity(size);

                for id in 0..size {
                    // create some threads and store them in the vector
                    // threads.push(Worker::new(id));
                    workers.push(Worker::new(id));
                }

                // ThreadPool { threads }
                ThreadPool { workers }
            }

            pub fn execute<F>(&self, f: F)
            where
                F: FnOnce() + Send + 'static,
            {
            }
        }

        // eg20-15, +
        // 1. Define a Worker struct that holds an id and a JoinHandle<()>.
        struct Worker {
            id: usize,
            thread: thread::JoinHandle<()>,
        }
        // 3. Define a Worker::new function that takes an id number and returns a Worker
        //    instance that holds the id and a thread spawned with an empty closure.
        impl Worker {
            pub fn new(id: usize) -> Self {
                Worker {
                    id,
                    thread: thread::spawn(|| {}),
                }
            }
        }
    }
} */
/* comiler ok, but the navigator is empty. */

/* // eg20-12, eg20-16 to eg20-17, with error
fn f10() {
    // eg20-12
    use hello::ThreadPool;
    use std::net::TcpListener;

    // let listener = TcpListener::bind(IP_AND_PORT).unwrap();
    let listener = TcpListener::bind("127.0.0.1:7888").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    mod hello {
        // eg20-16, +
        use std::{sync::mpsc, thread};
        struct Job;

        pub struct ThreadPool {
            workers: Vec<Worker>,

            // eg20-16, +
            sender: mpsc::Sender<Job>,
        }

        impl ThreadPool {
            pub fn new(size: usize) -> ThreadPool {
                assert!(size > 0);

                // -------------- eg20-17, remove, start
                // let mut workers = Vec::with_capacity(size);
                // for id in 0..size {
                //     workers.push(Worker::new(id));
                // }

                // // eg20-16, -
                // // ThreadPool { workers }

                // // eg20-16, +
                // let (sender, receiver) = mpsc::channel();
                // ThreadPool { workers, sender }
                // -------------- eg20-17, remove, end

                // -------------- eg20-17, +, start
                let (sender, receiver) = mpsc::channel();

                let mut workers = Vec::with_capacity(size);

                for id in 0..size {
                    workers.push(Worker::new(id, receiver));
                }

                ThreadPool { workers, sender }
                // -------------- eg20-17, +, end
            }

            pub fn execute<F>(&self, f: F)
            where
                F: FnOnce() + Send + 'static,
            {
            }
        }

        struct Worker {
            id: usize,
            thread: thread::JoinHandle<()>,
        }

        impl Worker {
            // -------------- eg20-17, remove, start
            // pub fn new(id: usize) -> Self {
            //     Worker {
            //         id,
            //         thread: thread::spawn(|| {}),
            //     }
            // }
            // -------------- eg20-17, remove, end

            // -------------- eg20-17, +, start
            fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
                let thread = thread::spawn(|| {
                    receiver;
                });

                Worker { id, thread }
            }
            // -------------- eg20-17, +, end
        }
    }
} */
/*
error[E0382]: use of moved value: `receiver`
   --> src\main.rs:928:50
    |
923 |                 let (sender, receiver) = mpsc::channel();
    |                              -------- move occurs because `receiver` has type `std::sync::mpsc::Receiver<Job>`, which does not implement the `Copy` trait
...
927 |                 for id in 0..size {
    |                 ----------------- inside of this loop
928 |                     workers.push(Worker::new(id, receiver));
    |                                                  ^^^^^^^^ value moved here, in previous iteration of loop
    |
note: consider changing this parameter type in method `new` to borrow instead if owning the value isn't necessary
   --> src\main.rs:958:41
    |
958 |             fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
    |                --- in this method       ^^^^^^^^^^^^^^^^^^^ this parameter takes ownership of the value
*/

// eg20-12, eg20-18 to eg20-21
fn f10() {
    // eg20-12 to eg20-21
    // eg20-20 is ok, but eg20-21 is wrong.

    // eg20-12
    use hello::ThreadPool;
    use std::net::TcpListener;

    // let listener = TcpListener::bind(IP_AND_PORT).unwrap();
    let listener = TcpListener::bind("127.0.0.1:7888").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    mod hello {
        // eg20-18, -
        // use std::{sync::mpsc, thread};

        // eg20-18, +
        use std::{
            sync::{mpsc, Arc, Mutex},
            thread,
        };

        // eg20-19, -
        // struct Job;
        // eg20-19, +
        type Job = Box<dyn FnOnce() + Send + 'static>;

        pub struct ThreadPool {
            workers: Vec<Worker>,
            sender: mpsc::Sender<Job>,
        }

        impl ThreadPool {
            pub fn new(size: usize) -> ThreadPool {
                assert!(size > 0);

                let (sender, receiver) = mpsc::channel();

                // -------------- eg20-18, remove, start
                // let mut workers = Vec::with_capacity(size);

                // for id in 0..size {
                //     workers.push(Worker::new(id, receiver));
                // }
                // -------------- eg20-18, remove, end

                // -------------- eg20-18, +, start
                let receiver = Arc::new(Mutex::new(receiver));

                let mut workers = Vec::with_capacity(size);
                for id in 0..size {
                    workers.push(Worker::new(id, Arc::clone(&receiver)));
                }
                // -------------- eg20-18, +, end

                ThreadPool { workers, sender }
            }

            pub fn execute<F>(&self, f: F)
            where
                F: FnOnce() + Send + 'static,
            {
                // eg20-19, +
                let job = Box::new(f);

                self.sender.send(job).unwrap();
            }
        }

        struct Worker {
            id: usize,
            thread: thread::JoinHandle<()>,
        }

        impl Worker {
            // eg20-18, -
            // fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {

            // eg20-18, +
            fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
                // -------------- eg20-20, remove, start
                //   let thread = thread::spawn(|| {
                //     receiver;
                //     // why suggest: use `drop` to clarify the intent: `drop(receiver);`
                //     // path statement drops value
                //     // `#[warn(path_statements)]` on by default
                //     // drop(receiver);
                // });
                // -------------- eg20-20, remove, end

                // -------------- eg20-20, +, start
                let thread = thread::spawn(move || loop {
                    let job = receiver.lock().unwrap().recv().unwrap();

                    println!("Worker {id} got a job; executing.");

                    job();
                });
                // -------------- eg20-20, +, end

                Worker { id, thread }
            }

            // This code compiles and runs but doesn’t result in the desired threading behavior:
            // a slow request will still cause other requests to wait to be processed.
            // Do not use this:
            // eg20-21
            // fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
            //   let thread = thread::spawn(move || {
            //       while let Ok(job) = receiver.lock().unwrap().recv() {
            //           println!("Worker {id} got a job; executing.");

            //           job();
            //       }
            //   });

            //   Worker { id, thread }
            // }
        }
    }
}
/*
Worker 0 got a job; executing.
Worker 3 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 0 got a job; executing.
Worker 3 got a job; executing.
Worker 2 got a job; executing.
thread '<unnamed>' panicked at src\main.rs:569:50:
called `Option::unwrap()` on a `None` value
*/
/* navigator: http://127.0.0.1:7888
Hello!
Hi from Rust
*/
/* navigator: http://127.0.0.1:7888/sleep (slow only this page)
Hello!
Hi from Rust
*/
/* navigator: http://127.0.0.1:7888/any
Oops!
Sorry, I don't know what you're asking for.
*/

/*
* <en>https://doc.rust-lang.org/book/ch20-03-graceful-shutdown-and-cleanup.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch20-03-graceful-shutdown-and-cleanup.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch20-03-graceful-shutdown-and-cleanup.html</tw>
*/

/* // eg20-22, with error
fn f11() {
    // begin from eg20-20

    use hello::ThreadPool;
    use std::net::TcpListener;

    // let listener = TcpListener::bind(IP_AND_PORT).unwrap();
    let listener = TcpListener::bind("127.0.0.1:7889").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    mod hello {
        use std::{
            sync::{mpsc, Arc, Mutex},
            thread,
        };

        type Job = Box<dyn FnOnce() + Send + 'static>;

        pub struct ThreadPool {
            workers: Vec<Worker>,
            sender: mpsc::Sender<Job>,
        }

        impl ThreadPool {
            pub fn new(size: usize) -> ThreadPool {
                assert!(size > 0);

                let (sender, receiver) = mpsc::channel();

                let receiver = Arc::new(Mutex::new(receiver));

                let mut workers = Vec::with_capacity(size);
                for id in 0..size {
                    workers.push(Worker::new(id, Arc::clone(&receiver)));
                }

                ThreadPool { workers, sender }
            }

            pub fn execute<F>(&self, f: F)
            where
                F: FnOnce() + Send + 'static,
            {
                let job = Box::new(f);

                self.sender.send(job).unwrap();
            }
        }

        struct Worker {
            id: usize,
            thread: thread::JoinHandle<()>,
        }

        impl Worker {
            fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
                let thread = thread::spawn(move || loop {
                    let job = receiver.lock().unwrap().recv().unwrap();

                    println!("Worker {id} got a job; executing.");

                    job();
                });

                Worker { id, thread }
            }
        }

        // eg20-22
        impl Drop for ThreadPool {
            fn drop(&mut self) {
                for worker in &mut self.workers {
                    println!("Shutting down worker {}", worker.id);

                    worker.thread.join().unwrap();
                }
            }
        }
    }
} */
/*
error[E0507]: cannot move out of `worker.thread` which is behind a mutable reference
    --> src\main.rs:1230:21
     |
1230 |                     worker.thread.join().unwrap();
     |                     ^^^^^^^^^^^^^ ------ `worker.thread` moved due to this method call
     |                     |
     |                     move occurs because `worker.thread` has type `JoinHandle<()>`, which does not implement the `Copy` trait
     |
note: `JoinHandle::<T>::join` takes ownership of the receiver `self`, which moves `worker.thread`
    --> D:\anqi\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\std\src\thread\mod.rs:1650:17
     |
1650 |     pub fn join(self) -> Result<T> {
*/

#[allow(unused_variables)]
#[allow(dead_code)]
fn f11() {
    // chapter20::eg22_next_with_error::f11();
    // chapter20::eg22_next_next_next_with_error::f11();

    // begin from eg20-20

    use hello::ThreadPool;
    use std::net::TcpListener;

    // let listener = TcpListener::bind(IP_AND_PORT).unwrap();
    let listener = TcpListener::bind("127.0.0.1:7889").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    mod hello {
        use std::{
            sync::{mpsc, Arc, Mutex},
            thread,
        };

        type Job = Box<dyn FnOnce() + Send + 'static>;

        pub struct ThreadPool {
            workers: Vec<Worker>,
            sender: mpsc::Sender<Job>,
        }

        impl ThreadPool {
            pub fn new(size: usize) -> ThreadPool {
                assert!(size > 0);

                let (sender, receiver) = mpsc::channel();

                let receiver = Arc::new(Mutex::new(receiver));

                let mut workers = Vec::with_capacity(size);
                for id in 0..size {
                    workers.push(Worker::new(id, Arc::clone(&receiver)));
                }

                ThreadPool { workers, sender }
            }

            pub fn execute<F>(&self, f: F)
            where
                F: FnOnce() + Send + 'static,
            {
                let job = Box::new(f);

                self.sender.send(job).unwrap();
            }
        }

        // struct Worker {
        //     id: usize,
        //     thread: thread::JoinHandle<()>,
        // }
        struct Worker {
            id: usize,
            thread: Option<thread::JoinHandle<()>>,
        }

        impl Worker {
            fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
                let thread = thread::spawn(move || loop {
                    let job = receiver.lock().unwrap().recv().unwrap();

                    println!("Worker {id} got a job; executing.");

                    job();
                });

                // Worker { id, thread }
                Worker {
                    id,
                    thread: Some(thread),
                }
            }
        }

        // eg20-22
        impl Drop for ThreadPool {
            fn drop(&mut self) {
                for worker in &mut self.workers {
                    println!("Shutting down worker {}", worker.id);

                    // worker.thread.join().unwrap();
                    if let Some(thread) = worker.thread.take() {
                        thread.join().unwrap();
                    }
                }
            }
        }
    }
}

fn f12() {
    // eg20-23 to eg20-25

    // chapter20::eg23::f12();

    use hello::ThreadPool;
    use std::net::TcpListener;

    // let listener = TcpListener::bind(IP_AND_PORT).unwrap();
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // eg20-25, -
    // for stream in listener.incoming() {
    // eg20-25, +
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");

    mod hello {
        use std::{
            sync::{mpsc, Arc, Mutex},
            thread,
        };

        type Job = Box<dyn FnOnce() + Send + 'static>;

        pub struct ThreadPool {
            workers: Vec<Worker>,

            // eg20-23, -
            // sender: mpsc::Sender<Job>,
            // eg20-23, +
            sender: Option<mpsc::Sender<Job>>,
        }

        impl ThreadPool {
            pub fn new(size: usize) -> ThreadPool {
                assert!(size > 0);

                let (sender, receiver) = mpsc::channel();

                let receiver = Arc::new(Mutex::new(receiver));

                let mut workers = Vec::with_capacity(size);
                for id in 0..size {
                    workers.push(Worker::new(id, Arc::clone(&receiver)));
                }

                // eg20-23, -
                // ThreadPool { workers, sender }
                // eg20-23, +
                ThreadPool {
                    workers,
                    sender: Some(sender),
                }
            }

            pub fn execute<F>(&self, f: F)
            where
                F: FnOnce() + Send + 'static,
            {
                let job = Box::new(f);

                // eg20-23, -
                // self.sender.send(job).unwrap();
                // eg20-23, +
                self.sender.as_ref().unwrap().send(job).unwrap();
            }
        }

        struct Worker {
            id: usize,
            thread: Option<thread::JoinHandle<()>>,
        }

        impl Worker {
            fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
                let thread = thread::spawn(move || loop {
                    // -------------------------------------------------- eg20-24, -, start
                    // let job = receiver.lock().unwrap().recv().unwrap();

                    // println!("Worker {id} got a job; executing.");

                    // job();
                    // -------------------------------------------------- eg20-24, -, end

                    // -------------------------------------------------- eg20-24, +, start
                    let message = receiver.lock().unwrap().recv();

                    match message {
                        Ok(job) => {
                            println!("Worker {id} got a job; executing.");

                            job();
                        }
                        Err(_) => {
                            println!("Worker {id} disconnected; shutting down.");
                            break;
                        }
                    }
                    // -------------------------------------------------- eg20-24, +, end
                });

                // eg20-23, -
                // Worker { id, thread }
                // eg20-23, +
                Worker {
                    id,
                    thread: Some(thread),
                }
            }
        }

        impl Drop for ThreadPool {
            fn drop(&mut self) {
                // eg20-23, +
                drop(self.sender.take());

                for worker in &mut self.workers {
                    println!("Shutting down worker {}", worker.id);

                    // worker.thread.join().unwrap();
                    if let Some(thread) = worker.thread.take() {
                        thread.join().unwrap();
                    }
                }
            }
        }
    }
}
/*
Worker 0 got a job; executing.
Shutting down.
Shutting down worker 0
Worker 1 got a job; executing.
Worker 3 disconnected; shutting down.
Worker 2 disconnected; shutting down.
Worker 0 disconnected; shutting down.
Shutting down worker 1
Worker 1 disconnected; shutting down.
Shutting down worker 2
Shutting down worker 3
*/
/* navigator: http://127.0.0.1:7888
Hello!
Hi from Rust
*/
/* navigator: http://127.0.0.1:7888/sleep (slow only this page)
Hello!
Hi from Rust
*/
/* navigator: http://127.0.0.1:7888/any
Oops!
Sorry, I don't know what you're asking for.
*/

fn main() {
    // eg20-1 http://127.0.0.1:7879
    // f01();

    // eg20-2 http://127.0.0.1:7880
    // f02();

    // eg20-3 http://127.0.0.1:7881
    // f03();

    // eg20-4 hello.html

    // eg20-5 http://127.0.0.1:7882
    // f04();

    // eg20-6 http://127.0.0.1:7883
    // f05();

    // eg20-7 http://127.0.0.1:7884
    // f06();

    // eg20-8 404.html

    // eg20-9 http://127.0.0.1:7885
    // f07();

    // eg20-10 http://127.0.0.1:7886
    // f08();

    // eg20-10 http://127.0.0.1:7887
    // f09();

    // eg20-11 http://127.0.0.1:7888
    // f10();

    // eg20-22 http://127.0.0.1:7889
    // f11();

    // eg20-25 http://127.0.0.1:7878
    f12();
}
