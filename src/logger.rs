use std::io::Write;

use colored::Colorize;
use log::{LevelFilter, SetLoggerError};
pub struct ServerLogger;

fn strip_color_characters(input: String) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        if ch == '\x1b' {
            chars.next();
            if let Some(&'[') = chars.peek() {
                chars.next(); 
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_alphabetic() {
                        chars.next(); // skip alphabetic characers
                        break;
                    } else {
                        chars.next();
                    }
                }
            }
        } else {
            result.push(ch);
            chars.next();
        }
    }

    result
}
pub static LOGGER: ServerLogger = ServerLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug))
}

impl log::Log for ServerLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Debug
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let level = match record.level() {
                log::Level::Error => "ERROR".bold().bright_red(),
                log::Level::Warn => "WARN".bold().bright_yellow(),
                log::Level::Info => "INFO".bold().bright_white(),
                log::Level::Debug => "DEBUG".bold().bright_blue(),
                log::Level::Trace => "TRACE".bold().italic().bright_black(),
            };
            let dt = chrono::Utc::now();
            let time = dt.format("%m:%d:%H:%M:%S");
            let thread_id = std::thread::current().id().as_u64();
            let mut log_line = String::new();
            use std::fmt::Write as FmtWrite; // this is bs rust what is this??? - caz
            write!(
                log_line,
                "[{} Thread#{}/{}] >> {}",
                time,
                thread_id,
                level,
                record.args()
            )
            .expect("Failed to write log");
            println!("{}", log_line);
            if let Ok(mut file) = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open("./server.log")
            {
                let mut stripped_line = strip_color_characters(log_line);
                stripped_line.push('\n'); // wont work if you use double quotes???
                if let Err(e) = file.write_all(stripped_line.as_bytes()) {
                    eprintln!("Failed to write logfile: {e}");
                }
            } else {
                eprintln!(
                    "{}",
                    "Unable to open or create logfile, is the server readable/writable?"
                        .red()
                        .bold()
                )
            }
        }
    }

    fn flush(&self) {}
}
