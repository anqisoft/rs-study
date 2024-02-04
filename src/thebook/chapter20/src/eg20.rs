pub fn f10() {
    // eg20-12 to eg20-21
    // eg20-20 is ok, but eg20-21 is wrong.

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
        use std::{
            sync::{mpsc, Arc, Mutex},
            thread,
        };

        type Job = Box<dyn FnOnce() + Send + 'static>;

        #[allow(dead_code)]
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

        #[allow(dead_code)]
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
