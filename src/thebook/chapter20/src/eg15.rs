pub fn f10() {
    // eg20-12
    use hello::ThreadPool;
    use std::net::TcpListener;

    // let listener = TcpListener::bind(IP_AND_PORT).unwrap();
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

        // 2. Change ThreadPool to hold a vector of Worker instances.
        #[allow(dead_code)]
        pub struct ThreadPool {
            workers: Vec<Worker>,
        }

        impl ThreadPool {
            // 4. In ThreadPool::new, use the for loop counter to generate an id,
            //    create a new Worker with that id, and store the worker in the vector.
            pub fn new(size: usize) -> ThreadPool {
                assert!(size > 0);

                let mut workers = Vec::with_capacity(size);

                for id in 0..size {
                    // create some threads and store them in the vector
                    workers.push(Worker::new(id));
                }

                ThreadPool { workers }
            }

            #[allow(unused_variables)]
            pub fn execute<F>(&self, f: F)
            where
                F: FnOnce() + Send + 'static,
            {
            }
        }

        // 1. Define a Worker struct that holds an id and a JoinHandle<()>.
        #[allow(dead_code)]
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
}
/* comiler ok, but the navigator is empty. */
