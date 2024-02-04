pub fn f10() {
    // eg20-12
    use hello::ThreadPool;
    use std::net::TcpListener;

    let listener = TcpListener::bind("127.0.0.1:7888").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            crate::common::handle_connection(stream);
        });
    }
    println!("Shutting down.");

    mod hello {
        use std::thread;
        #[allow(dead_code)]
        pub struct ThreadPool {
            threads: Vec<thread::JoinHandle<()>>,
        }

        impl ThreadPool {
            pub fn new(size: usize) -> ThreadPool {
                assert!(size > 0);

                #[allow(unused_mut)]
                let mut threads = Vec::with_capacity(size);

                for _ in 0..size {
                    // create some threads and store them in the vector
                }

                ThreadPool { threads }
            }

            #[allow(unused_variables)]
            pub fn execute<F>(&self, f: F)
            where
                F: FnOnce() + Send + 'static,
            {
            }
        }
    }
}
/* compile ok */
/*
warning: variable does not need to be mutable
   --> src\main.rs:27:21
    |
27  |                 let mut threads = Vec::with_capacity(size);
    |                     ----^^^^^^^
    |                     |
    |                     help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` on by default
*/
