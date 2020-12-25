use std::thread::{Thread, JoinHandle};
use std::sync::mpsc;
use std::thread;
use crate::core::common::{Event, CommandCollection};
use crate::core::orchestration::CommandHandler;

pub struct EventChannel {
    pub(crate) sender: mpsc::Sender<Box<dyn Event + Send>>
}

pub struct EventLoop {
    handler: JoinHandle<()>
}

impl EventLoop {

    fn start(receiver: mpsc::Receiver<Box<dyn Event + Send>>, comm_handler: CommandHandler) -> Result<EventLoop, &'static str> {

        let handler = thread::spawn(move || loop {
            let event = receiver.recv().unwrap();
            comm_handler.send(event.handle(None));
        });

        Ok(EventLoop {
            handler
        })
    }
}

impl EventChannel {
    /// Clones the internal sender.
    pub fn get(&self) -> mpsc::Sender<Box<dyn Event + Send>> {
        self.sender.clone()
    }
} 