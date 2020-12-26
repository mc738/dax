use uuid::Uuid;
use dax::core::logging::{Log, LogItem};
use dax::core::common::{JobResult, Job};
use dax::app::Engine;
use dax::core::jobs::{ProcSettings};


fn main() {
    
   let proc = ProcSettings::create("/home/max/Projects/Data".to_string(), None);

    let job = proc.create_job();
    let job2 = proc.create_job();
    
    let result = job();
    
    let result = job2();
    
    let engine = Engine::create().unwrap();

    loop {
     
    }
    
    // println!("Hello, world!");
}
