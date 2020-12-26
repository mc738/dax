use std::thread::{Thread, JoinHandle};
use std::sync::mpsc;
use std::thread;
use crate::core::common::{Event, CommandCollection};
use crate::core::orchestration::CommandHandler;
use crate::core::logging::{Logger, LogItem};

pub struct EventChannel {
    pub(crate) sender: mpsc::Sender<Box<dyn Event + Send>>
}

pub struct EventLoop {
    handler: JoinHandle<()>
}

impl EventLoop {

    pub fn create(receiver: mpsc::Receiver<Box<dyn Event + Send>>, comm_handler: CommandHandler, logger: Logger) -> Result<EventLoop, &'static str> {

        logger.log(LogItem::info("event_loop".to_string(), "Starting...".to_string()));


        logger.log(LogItem::success("event_loop".to_string(), "Started successfully".to_string()));
        
        let handler = thread::spawn(move || loop {
            let event = receiver.recv().unwrap();
            
            logger.log(LogItem::info("event_loop".to_string(), "Event received".to_string()));
            
            comm_handler.send(event.handle(None));

            logger.log(LogItem::success("event_loop".to_string(), "Event successfully handled".to_string()));
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