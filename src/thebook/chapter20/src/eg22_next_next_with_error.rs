// eg20-22, with error
pub fn f11() {
    // begin from eg20-20

    use hello::ThreadPool;
    use std::net::TcpListener;

    // let listener = TcpListener::bind(IP_AND_PORT).unwrap();
    let listener = TcpListener::bind("127.0.0.1:7889").unwrap();
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

                    worker.thread.join().unwrap();
                }
            }
        }
    }
}
/*
error[E0599]: no method named `join` found for enum `Option` in the current scope
    --> src\eg22_next_next_with_error.rs:92:35
     |
92   |                     worker.thread.join().unwrap();
     |                                   ^^^^ method not found in `Option<JoinHandle<()>>`
     |
note: the method `join` exists on the type `JoinHandle<()>`
    --> D:\anqi\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\std\src\thread\mod.rs:1650:5
     |
1650 |     pub fn join(self) -> Result<T> {
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider using `Option::expect` to unwrap the `JoinHandle<()>` value, panicking if the value is an `Option::None`
     |
92   |                     worker.thread.expect("REASON").join().unwrap();
*/