use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    // fn new(id: usize) -> Worker {
    //     let thread = thread::spawn(move || {

    //     });
    //     Worker {
    //         id,
    //         thread,
    //     }
    // }
    // fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a job!", id);
                job();
            }
        });
        Worker {
            id,
            thread,
        }
    }
}

pub struct ThreadPool {
    // threads: Vec<thread::JoinHandle<()>>,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

// struct Job;
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        // let mut threads = Vec::with_capacity(size);
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver)); // 线程安全


        // for _ in 0..size {
        //     let handle = thread::spawn(move || {

        //     });
        //     threads.push(handle);
        // }

        for id in 0..size {
            // workers.push(Worker::new(id));
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            // threads
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}