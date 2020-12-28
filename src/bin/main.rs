use uuid::Uuid;
use dax::core::logging::{Log, LogItem};
use dax::core::common::{JobResult, Job, Command, Action, CommandCollection};
use dax::app::Engine;
use dax::core::jobs::{ProcSettings};


struct TestCommand {
    task_id: Uuid
    
    
}

struct TestLog {
    from: String
}

//struct TestLog {

//}

impl TestCommand {
    pub fn create(task_id: Uuid) -> TestCommand {
        TestCommand { task_id }
    }
}

impl Command for TestCommand {
    
    fn create_actions(&self, task_id: Uuid) -> Vec<Action> {
        let mut actions = Vec::new();


        let proc = ProcSettings::create("/home/max/Projects/Data".to_string(), None);
        let job = proc.create_job();

        actions.push(Action::Job(job));
        actions.push(Action::Log(LogItem::info("test".to_string(), "Attempting to log via act action".to_string())));
        actions.push(Action::Log(LogItem::success("test".to_string(), "It worked!".to_string())));


        actions
    }
}

fn get_banner() -> &'static str {
    "\
\x1B[35m ____________       _____       _____       _____  \x1B[32m     _.--._       /|
\x1B[35m \\           \\    /      |_     \\    \\     /    / \x1B[32m    .'()..()`.    / /
\x1B[35m  \\           \\  /         \\     \\    |   |    /   \x1B[32m  ( `-.__.-' )  ( (
\x1B[35m   |    /\\     ||     /\\    \\     \\    \\ /    /    \x1B[32m   \\        /    \\ \\
\x1B[35m   |   |  |    ||    |  |    \\     \\    |    /    \x1B[32m     \\      /      ) )
\x1B[35m   |    \\/     ||     \\/      \\    /    |    \\  \x1B[32m     .'\x1B[33m -.__.-\x1B[32m `.-.-'_.'
\x1B[35m  /           /||\\      /\\     \\  /    /|\\    \\  \x1B[32m  .'  /\x1B[33m-____-\x1B[32m\\  `.-'
\x1B[35m /___________/ || \\_____\\ \\_____\\|____|/ \\|____| \x1B[32m  \\  /\x1B[33m-.____.-\x1B[32m\\  /-.
\x1B[35m|           | / | |     | |     ||    |   |    |   \x1B[32m \\ \\\x1B[33m`-.__.-'\x1B[32m/ /\\|\\|
\x1B[35m|___________|/   \\|_____|\\|_____||____|   |____|  \x1B[32m .'  `.    .'  `.
\x1B[35m                                                  \x1B[32m |/\\/\\|    |/\\/\\|\x1B[0m"
}

fn main() {


    println!("{}", get_banner());
    
    let proc = ProcSettings::create("/home/max/Projects/Data".to_string(), None);

    let job = proc.create_job();
    let job2 = proc.create_job();

    let result = job();

    let result = job2();

    let engine = Engine::create().unwrap();

    
    let comms_handler = engine.get_command_handler();
    
    let comms = CommandCollection::create(Uuid::new_v4(), vec![Box::new(TestCommand::create(Uuid::new_v4()))]);
    
    comms_handler.send(comms);
    
    loop {}

    // println!("Hello, world!");
}
