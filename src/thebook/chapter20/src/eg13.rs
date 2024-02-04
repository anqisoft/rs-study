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
        pub struct ThreadPool;

        impl ThreadPool {
            /// Create a new ThreadPool.
            ///
            /// The size is the number of threads in the pool.
            ///
            /// # Panics
            ///
            /// The `new` function will panic if the size is zero.
            pub fn new(size: usize) -> ThreadPool {
                assert!(size > 0);

                ThreadPool
            }
            // TODO
            // pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> { }

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
