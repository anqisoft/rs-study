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
}
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
