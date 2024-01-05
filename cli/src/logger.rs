use console::{style, Color};

pub struct ConsoleLogger;

static LOGGER: ConsoleLogger = ConsoleLogger;

impl ConsoleLogger {
    pub fn init() {
        log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Info));
    }
}

impl log::Log for ConsoleLogger {
    fn log(&self, record: &log::Record) {
        /*let colour = match record.level() {
            log::Level::Info => (x) => style(x).blue(),
            _ => (x) => style(x).white()
        }*/

        let colour = match record.level() {
            log::Level::Info => Color::Green,
            log::Level::Warn => Color::Yellow,
            log::Level::Error => Color::Red,
            log::Level::Debug => Color::Blue,
            _ => Color::Black,
        };

        //let message = format!("{:07}", record.args());

        let level = format!("{: <5}", record.level());
        println!(
            "{} {}",
            style(level).fg(colour).bold(),
            //style(level).bg(colour).fg(Color::Black).bold(),
            record.args()
        );
    }

    fn flush(&self) {}

    fn enabled(&self, metadata: &log::Metadata) -> bool {
        true
    }
}
