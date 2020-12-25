use uuid::Uuid;
use dax::core::logging::{Log, LogItem};
use dax::core::common::{JobResult, Job};

struct Proc {
    
}

impl Job for Proc {
    fn execute(&self, task_id: Uuid) -> JobResult {
        println!("Executing process (task_id: {})", task_id);
        unimplemented!()
    }
}




fn main() {
    
    // println!("Hello, world!");
}
