use uuid::Uuid;

pub struct Task {
    id: Uuid
}

pub trait Event {
    /// Handle the event to product a collection of objects implementing the `command` trait.
    /// If a `task_id` is provided, it will be used. If not a new `uuid` will be generated. 
    fn handle(&self, task_id: Option<Uuid>) -> CommandCollection;
}

pub struct CommandCollection {
    task_id: Uuid,
    commands: Vec<Box<dyn Command + Send>>,
}

pub trait Command {
    /// Handle the company to produce an executable action.
    /// The `orchestration` thread while use this to handle commands.
    fn handle(&self, task_id: Uuid) -> Action;
}

pub enum Action {
    Log,
    DbWrite,
    Job,
    IO
}

pub trait Job {
    /// Execute the job and return a `JobResult`.
    /// This can the be handled by the assigned `Worker`.
    fn execute(&self, task_id: Uuid) -> JobResult;
}

pub struct JobResult {
    job_id: Uuid,
    success: bool,
    commands: Option<CommandCollection>,
    events: Option<Vec<Box<dyn Event>>>,
}

impl CommandCollection {
    
    pub fn to_actions(&self) -> Vec<Action> {
        
        let mut actions = Vec::new();
        
        for comm in &self.commands {
            actions.push(comm.handle(self.task_id));
        }
        
        
        actions
    }
}