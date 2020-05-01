use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            let handle = thread::spawn(move || {

            });
            threads.push(handle);
        }
        ThreadPool {
            threads
        }
    }

    pub fn execute<F>(&self, f: F) \
        where F: FnOnce() + Send + 'static {

    }
}