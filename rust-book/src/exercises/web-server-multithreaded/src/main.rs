use std::{
    io::{BufReader, prelude::*}, net::{TcpListener, TcpStream}, sync::{Arc, Mutex, mpsc::Receiver}, thread::{self, JoinHandle}
};

use std::{sync::mpsc};
pub struct ThreadPool {
    pub workers: Vec<Worker>,
    pub sender: mpsc::Sender<Job>
}

struct Job;

pub struct Worker {
    pub id: usize,
    pub thread: JoinHandle<Arc<Mutex<Receiver<Job>>>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            receiver
        });

        Worker { id, thread }
    }
}

impl ThreadPool {
    pub fn new(workers: usize) -> ThreadPool {
        assert!(workers > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut total_workers = Vec::with_capacity(workers);

        for id in 0..workers {
            total_workers.push(Worker::new(id, Arc::clone(&receiver)));
        };

        ThreadPool { workers: total_workers, sender: sender }
    }

    pub fn execute<F>(&self, f: F)
    where F: 
        FnOnce() + Send + 'static,
     {
        let job = Box::new(f);

        self.sender.send(job).unwrap();    
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8001").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
        pool.execute(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();


    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
    println!("Request: {http_request:#?}");
}