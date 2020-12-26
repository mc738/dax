use uuid::Uuid;
use dax::core::logging::{Log, LogItem};
use dax::core::common::{JobResult, Job};
use dax::app::Engine;
use dax::core::jobs::{ProcSettings};


fn main() {
    
   let proc = ProcSettings::create("git".to_string(), None);
    
    let job = proc.create_job();
    
    let result = job();
    
    let engine = Engine::start().unwrap();

    
    
    // println!("Hello, world!");
}
