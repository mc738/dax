use std::sync::mpsc::{Sender, Receiver, channel};
use crate::core::logging::{Log, LogItem, Logger};
use crate::core::events::EventLoop;
use crate::core::orchestration::{Orchestrator, CommandHandler};
use crate::core::common::{CommandCollection, Event};

pub struct Engine {
    log: Log,
    event_loop: EventLoop,
    orchestrator: Orchestrator,
}

impl Engine {
    pub fn start() -> Result<Engine, &'static str> {

        let (log_send, log_receive) : (Sender<LogItem>, Receiver<LogItem>) = channel();
        let (com_send, com_receive) : (Sender<CommandCollection>, Receiver<CommandCollection>) = channel();
        let (evt_send, evt_receive) : (Sender<Box<dyn Event + Send>>, Receiver<Box<dyn Event + Send>>) = channel();
        
        let logger = Logger::create(log_send);
        let comm_handler = CommandHandler::create(com_send);
        
        // Create communication channels
        
        // Start logger
        let log = Log::create(log_receive)?;
        
        // Start orchestrator
        let orchestrator = Orchestrator::create(com_receive,  logger.clone())?;
        
        // Start event loop
        let event_loop = EventLoop::create(evt_receive, comm_handler.clone(), logger.clone())?;
        
        // Start workers


        loop {

        }
        
        Ok(Engine {
            log,
            event_loop,
            orchestrator
        })
        
    }
}