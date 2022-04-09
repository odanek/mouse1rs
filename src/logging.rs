use log::{LevelFilter, Log, Metadata, Record};

#[cfg(debug_assertions)]
const LOG_LEVEL: LevelFilter = LevelFilter::Warn;

#[cfg(not(debug_assertions))]
const LOG_LEVEL: LevelFilter = LevelFilter::Error;

pub struct Logger;

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: Logger = Logger;

// TODO Should be probably part of Quad
pub fn init_logging() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LOG_LEVEL))
        .expect("Unable to initialize logging");
}
