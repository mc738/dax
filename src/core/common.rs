use uuid::Uuid;

pub struct Task {
    id: Uuid
}

//type Job = Box<dyn FnOnce() + Send + 'static>;

pub trait Event {
    /// Handle the event to product a collection of objects implementing the `command` trait.
    /// If a `task_id` is provided, it will be used. If not a new `uuid` will be generated. 
    fn handle(&self, task_id: Option<Uuid>) -> CommandCollection;
}

pub struct CommandCollection {
    pub task_id: Uuid,
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

pub type Job = Box<dyn FnOnce() -> JobResult + Send + 'static>;

pub trait Executable {
    /// Execute the job and return a `JobResult`.
    /// This can the be handled by the assigned `Worker`.
    fn execute(&self, task_id: Uuid) -> JobResult;
}

pub trait JobHandler {
    fn create_job(&self) -> Job;
}

pub struct JobResult {
    pub(crate) job_id: Uuid,
    pub(crate) success: bool,
    pub(crate) commands: Option<CommandCollection>,
    pub(crate) events: Option<Vec<Box<dyn Event>>>,
}

impl CommandCollection {
    
    pub fn to_actions(&self) -> Vec<Action> {
        
        let mut actions = Vec::new();
        
        for comm in &self.commands {
            actions.push(comm.handle(self.task_id));
        }
        
        
        actions
    }
    
    pub fn count(&self) -> usize {
        self.commands.len()
    }
}