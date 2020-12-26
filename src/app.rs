use std::sync::mpsc::{Sender, Receiver, channel};
use crate::core::logging::{Log, LogItem, Logger};
use crate::core::events::{EventLoop, EventChannel};
use crate::core::orchestration::{Orchestrator, CommandHandler};
use crate::core::common::{CommandCollection, Event};
use crate::core::workers::WorkerPool;

pub struct Engine {
    log: Log,
    event_loop: EventLoop,
    orchestrator: Orchestrator,
    workers: WorkerPool,
    logger: Logger,
    evt_channel: EventChannel,
    comm_handler: CommandHandler
}

impl Engine {
    pub fn create() -> Result<Engine, &'static str> {

        let (log_send, log_receive) : (Sender<LogItem>, Receiver<LogItem>) = channel();
        let (com_send, com_receive) : (Sender<CommandCollection>, Receiver<CommandCollection>) = channel();
        let (evt_send, evt_receive) : (Sender<Box<dyn Event + Send>>, Receiver<Box<dyn Event + Send>>) = channel();
        
        let logger = Logger::create(log_send);
        let comm_handler = CommandHandler::create(com_send);
        let evt_channel = EventChannel::create(evt_send);
        
        // Create communication channels
        
        // Start logger
        let log = Log::create(log_receive)?;
        
        // Start orchestrator
        let orchestrator = Orchestrator::create(com_receive,  logger.clone())?;
        
        // Start event loop
        let event_loop = EventLoop::create(evt_receive, comm_handler.clone(), logger.clone())?;
        
        // Start workers
        let workers = WorkerPool::new(4, logger.clone(), evt_channel.clone(), comm_handler.clone())?;
        
        Ok(Engine {
            log,
            event_loop,
            orchestrator,
            workers,
            logger,
            evt_channel,
            comm_handler
        })
        
    }
    
    pub fn start(&self) {
        loop {
            
        }
    }
    
    pub fn get_logger(&self) -> Logger {
        self.logger.clone()
    }
    
    pub fn get_events_channel(&self) -> EventChannel {
        self.evt_channel.clone()
    }
    
    pub fn get_command_handler(&self) -> CommandHandler {
        self.comm_handler.clone()
    }
}