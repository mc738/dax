use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{Sender, Receiver};
use chrono::Utc;
use crate::core::common::{Job, JobResult};
use crate::core::logging::{LogItem, Logger};
use crate::core::events::EventChannel;
use crate::core::orchestration::CommandHandler;

pub struct WorkerPool {
    logger: Logger,
    sender: mpsc::Sender<Job>,
    workers: Vec<Worker>,
}


struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl WorkerPool {
    pub fn new(size: usize, logger: Logger, event_channel: EventChannel, comm_handler: CommandHandler) -> Result<WorkerPool, &'static str> {
        assert!(size > 0);
        match size {
            0 => {
                Err("Thread pool size needs to be greater than 0")
            }
            _ => {
                let (sender, receiver) = mpsc::channel();

                let receiver = Arc::new(Mutex::new(receiver));

                let mut workers = Vec::with_capacity(size);

                for i in 0..size {
                    workers.push(
                        Worker::new(
                            i,
                            Arc::clone(&receiver),
                            logger.clone(),
                            event_channel.clone(),
                            comm_handler.clone()));
                }

                Ok(WorkerPool {
                    logger,
                    workers,
                    sender,
                })
            }
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() -> JobResult + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}


impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>, logger: Logger, evt_channel: EventChannel, comm_handler: CommandHandler) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            logger.log(LogItem::info(format!("thread_{}", id), String::from("Job received")));

            // println! ("Worker {} got a job; executing.", id);
            let result = job();

            match result.commands {
                Some(comms) => comm_handler.send(comms),
                None => {}
            }

            match result.events {
                Some(evts) => {
                    for evt in evts {
                        evt_channel.send(evt);
                    }
                }
                None => {}
            };


            // Add log item

            // Handle job result actions and events.
        });

        Worker { id, thread }
    }
}

