pub fn f12() {
    // begin from eg20-23

    use hello::ThreadPool;
    use std::net::TcpListener;

    // let listener = TcpListener::bind(IP_AND_PORT).unwrap();
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
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
                    let job = receiver.lock().unwrap().recv().unwrap();

                    println!("Worker {id} got a job; executing.");

                    job();
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
/* compiler and run, ok.
    Navigator is not work.
*/
/*

*/
