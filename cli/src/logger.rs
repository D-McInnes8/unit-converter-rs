use console::{style, Color};

use crate::options::LogLevel;

pub struct ConsoleLogger {
    enabled: bool,
}

static mut LOGGER: ConsoleLogger = ConsoleLogger { enabled: true };

impl ConsoleLogger {
    pub fn init(log_level: &Option<LogLevel>) {
        if log_level.is_none() {
            unsafe {
                LOGGER.enabled = false;
            }
        }

        let level_filter = match log_level {
            Some(LogLevel::Trace) => log::LevelFilter::Trace,
            Some(LogLevel::Debug) => log::LevelFilter::Debug,
            Some(LogLevel::Info) => log::LevelFilter::Info,
            Some(LogLevel::Warning) => log::LevelFilter::Warn,
            Some(LogLevel::Error) => log::LevelFilter::Error,
            None => log::LevelFilter::Error,
        };

        unsafe {
            _ = log::set_logger(&LOGGER).map(|()| log::set_max_level(level_filter));
        }
    }
}

impl log::Log for ConsoleLogger {
    fn log(&self, record: &log::Record) {
        if !self.enabled {
            return;
        }

        let colour = match record.level() {
            log::Level::Info => Color::Green,
            log::Level::Warn => Color::Yellow,
            log::Level::Error => Color::Red,
            log::Level::Debug => Color::Blue,
            _ => Color::Black,
        };

        let level = format!("{: <5}", record.level());
        println!("{} {}", style(level).fg(colour).bold(), record.args());
    }

    fn flush(&self) {}

    fn enabled(&self, _: &log::Metadata) -> bool {
        self.enabled
    }
}
