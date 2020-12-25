use std::thread;
use std::sync::mpsc;
use std::thread::JoinHandle;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct Logger {
    sender: mpsc::Sender<LogItem>
} 

pub struct Log {
    handler: JoinHandle<()>
}

pub struct LogItem {
    pub(crate) from: String,
    pub(crate) message: String,
    pub(crate) item_type: LogItemType
}

pub enum LogItemType {
    Information,
    Success,
    Error,
    Warning,
    Debug,
}

pub enum ConsoleColor {
    Black,
    BlackBright,
    Red,
    RedBright,
    Green,
    GreenBright,
    Yellow,
    YellowBright,
    Blue,
    BlueBright,
    Magenta,
    MagentaBright,
    Cyan,
    CyanBright,
    White,
    WhiteBright,
    // Custom(i8)
}

impl Logger {
    pub fn create(sender: mpsc::Sender<LogItem>) -> Logger {
        Logger {
            sender
        }
    }
    
    pub fn log(&self, item:LogItem) {
        self.sender.send(item);
    }
}

impl Log {
    pub fn create(receiver: mpsc::Receiver<LogItem>) -> Result<Log, &'static str> {

        ConsoleColor::White.set_foreground();
        println!("[{} info  ] {} {}", Utc::now().format("%F %H:%M:%S%.3f"), "logger", "Starting...");
        ConsoleColor::reset();
        
        let handler = thread::spawn(move || loop {
            let item = receiver.recv().unwrap();

            let (color, name) = match item.item_type {
                LogItemType::Information => (ConsoleColor::White, "info  "),
                LogItemType::Success => (ConsoleColor::Green, "ok    "),
                LogItemType::Error => (ConsoleColor::Red, "error "),
                LogItemType::Warning => (ConsoleColor::Yellow, "warn  "),
                LogItemType::Debug => (ConsoleColor::MagentaBright, "debug "),
            };

            color.set_foreground();

            println!("[{} {}] {} {}", Utc::now().format("%F %H:%M:%S%.3f"), name, item.from, item.message);
            ConsoleColor::reset();

        });


        ConsoleColor::Green.set_foreground();
        println!("[{} ok    ] {} {}", Utc::now().format("%F %H:%M:%S%.3f"), "logger", "Started successfully");
        ConsoleColor::reset();

        Ok(Log { handler })
    }
}

impl LogItem {

    pub fn info(from: String, message: String) -> LogItem {
        LogItem::create_item(from, message, LogItemType::Information)
    }

    pub fn success(from: String, message: String) -> LogItem {
        LogItem::create_item(from, message, LogItemType::Success)
    }

    pub fn error(from: String, message: String) -> LogItem {
        LogItem::create_item(from, message, LogItemType::Error)
    }

    pub fn warning(from: String, message: String) -> LogItem {
        LogItem::create_item(from, message, LogItemType::Warning)
    }

    pub fn debug(from: String, message: String) -> LogItem {
        LogItem::create_item(from, message, LogItemType::Debug)
    }

    fn create_item(from: String, message: String, item_type: LogItemType) -> LogItem {
        LogItem { from, message, item_type }
    }
}


impl ConsoleColor {

    pub fn set_foreground(&self) {
        print!("{}", self.get_foreground_color())
    }

    pub fn set_background(&self) {
        print!("{}", self.get_background_color())
    }

    pub fn reset() {
        print!("\x1B[0m")
    }

    pub fn get_foreground_color(&self) -> &'static str {
        match self {
            ConsoleColor::Black => "\x1B[30m",
            ConsoleColor::BlackBright => "\x1B[30;1m",
            ConsoleColor::Red => "\x1B[31m",
            ConsoleColor::RedBright => "\x1B[31;1m",
            ConsoleColor::Green => "\x1B[32m",
            ConsoleColor::GreenBright => "\x1B[32;1m",
            ConsoleColor::Yellow => "\x1B[33m",
            ConsoleColor::YellowBright => "\x1B[33;1m",
            ConsoleColor::Blue => "\x1B[34m",
            ConsoleColor::BlueBright => "\x1B[34;1m",
            ConsoleColor::Magenta => "\x1B[35m",
            ConsoleColor::MagentaBright => "\x1B[35;1m",
            ConsoleColor::Cyan => "\x1B[36m",
            ConsoleColor::CyanBright => "\x1B[36m;1m",
            ConsoleColor::White => "\x1B[37m",
            ConsoleColor::WhiteBright => "\x1B[37;1m",
            //ConsoleColor::Custom(id) => format!("\x1B[38;5;${}m", id).as_str(),
        }
    }

    pub fn get_background_color(&self) -> &'static str {
        match &self {
            ConsoleColor::Black => "\x1B[40m",
            ConsoleColor::BlackBright => "\x1B[40;1m",
            ConsoleColor::Red => "\x1B[41m",
            ConsoleColor::RedBright => "\x1B[41;1m",
            ConsoleColor::Green => "\x1B[42m",
            ConsoleColor::GreenBright => "\x1B[42;1m",
            ConsoleColor::Yellow => "\x1B[43m",
            ConsoleColor::YellowBright => "\x1B[43;1m",
            ConsoleColor::Blue => "\x1B[44m",
            ConsoleColor::BlueBright => "\x1B[44;1m",
            ConsoleColor::Magenta => "\x1B[45m",
            ConsoleColor::MagentaBright => "\x1B[45;1m",
            ConsoleColor::Cyan => "\x1B[46m",
            ConsoleColor::CyanBright => "\x1B[46m;1m",
            ConsoleColor::White => "\x1B[47m",
            ConsoleColor::WhiteBright => "\x1B[47;1m",
            //ConsoleColor::Custom(id) => format!("\x1b[48;5;${}m", id).as_str(),
        }
    }
}
