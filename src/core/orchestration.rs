use std::sync::mpsc;
use crate::core::common::CommandCollection;
use std::thread::JoinHandle;
use crate::core::logging::{Logger, LogItem};
use std::thread;

#[derive(Clone)]
pub struct CommandHandler {
    pub(crate) sender: mpsc::Sender<CommandCollection>
}

pub struct Orchestrator {
    logger: Logger,
    handler: JoinHandle<()>
}

impl CommandHandler {
    pub fn create(sender: mpsc::Sender<CommandCollection>) -> CommandHandler {
        CommandHandler {
            sender
        }
    }
    
    pub fn send(&self, commands: CommandCollection) {
        self.sender.send(commands);
    }
   
}

impl Orchestrator {
    pub fn create(receiver: mpsc::Receiver<CommandCollection>, logger: Logger) -> Result<Orchestrator,&'static str> {
        logger.log(LogItem::info("orchestrator".to_string(), "Starting...".to_string()));

        let handler = thread::spawn(move || loop {
            let item = receiver.recv().unwrap();
            
            });
        
        
        Ok(Orchestrator { logger, handler })
        
        
    }
}