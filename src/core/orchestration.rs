use std::sync::mpsc;
use crate::core::common::{CommandCollection, Job, Action};
use std::thread::JoinHandle;
use crate::core::logging::{Logger, LogItem};
use std::thread;

#[derive(Clone)]
pub struct CommandHandler {
    pub(crate) sender: mpsc::Sender<CommandCollection>
}


pub struct Orchestrator {
    handler: JoinHandle<()>,
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
    
    pub fn create(receiver: mpsc::Receiver<CommandCollection>, logger: Logger, workers: mpsc::Sender<Job>) -> Result<Orchestrator, &'static str> {
        logger.log(LogItem::info("orchestrator".to_string(), "Starting...".to_string()));

        logger.log(LogItem::success("orchestrator".to_string(), "Started successfully".to_string()));
        
        let handler = thread::spawn(move || loop {
            let commands = receiver.recv().unwrap();
            
            let message = format!("Commands received (task_id: {}, count: {})", commands.task_id, commands.count());
            
            logger.log(LogItem::info("orchestrator".to_string(), message));
            
            for action in commands.to_actions() {
                match action {
                    Action::Log(item) => logger.log(item),
                    Action::DbWrite => {}
                    Action::Job(job) => workers.send(job).unwrap(),
                    Action::IO => {}
                }
            }

            let message = format!("Commands handled successfully (task_id: {}, count: {})", commands.task_id, commands.count());

            logger.log(LogItem::success("orchestrator".to_string(), message));
        
        });



        Ok(Orchestrator { handler })
    }
}