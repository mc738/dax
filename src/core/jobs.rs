use crate::core::common::{Job, JobResult};
use uuid::Uuid;
use std::process::Command;

#[derive(Clone)]
pub struct ProcSettings {
    command: String,
    args: Option<Vec<String>>,
    std_out_grep: Option<String>,
    std_err_grep: Option<String>,
}

#[derive(Debug)]
pub struct ProcResult {
    success: bool,
    pub(crate) out: Vec<u8>
}

impl ProcSettings {

    pub fn create(command: String, args: Option<Vec<String>>) -> ProcSettings {
        ProcSettings {
            command,
            args,
            std_out_grep: None,
            std_err_grep: None,
        }
    }
    
    pub fn create_job(&self) -> Job {
        
        let settings = self.clone();
        
        let handler = move || {

            let result = settings.run();
            println!("Result: {}", String::from_utf8(result.out).unwrap());
            
            JobResult {
                job_id: Default::default(),
                success: false,
                commands: None,
                events: None
            }
        };

        let job: Job = Box::new(handler);
        
        job
    }

    pub fn run(&self) -> ProcResult {
        let mut proc = Command::new(&self.command);

        if let Some(args) = &self.args {
            for arg in args {
                proc.arg(arg);
            }
        }

        let result = proc.output().expect(
            format!("Failed to start process `{}`", self.command).as_str());

        match result.status.success() {
            true => {
                ProcResult {
                    success: true,
                    out: result.stdout
                }
            }
            false => {
                ProcResult {
                    success: true,
                    out: result.stderr
                }
            }
        }
    }
}
