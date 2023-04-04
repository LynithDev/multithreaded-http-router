use std::{sync::{Arc, mpsc, Mutex}, thread};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    job();
                },
                Message::Terminate => {
                    break;
                },
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

pub enum Message {
    NewJob(Job),
    Terminate,
}