extern crate ansi_term;
extern crate floating_duration;
extern crate log;

use self::ansi_term::Color;
use self::log::{Level, Log, Metadata, Record, SetLoggerError};
use std::fmt;
use std::time::Instant;

struct ColorLevel(Level);

impl fmt::Display for ColorLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Level::Trace => Color::Purple.paint("TRACE"),
            Level::Debug => Color::Blue.paint("DEBUG"),
            Level::Info => Color::Green.paint("INFO"),
            Level::Warn => Color::Yellow.paint("WARNING"),
            Level::Error => Color::Red.paint("ERROR"),
        }
        .fmt(f)
    }
}

struct ScrubLog {
    init_time: Instant,
}

impl Log for ScrubLog {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let line_string = match record.line() {
                Some(x) => format!("{}", x),
                None => String::from("-"),
            };

            let timestamp = Instant::now() - self.init_time;
            println!(
                "{} {} [{}:{}] {}",
                ColorLevel(record.level()),
                floating_duration::TimeFormat(timestamp),
                record.module_path().unwrap_or("-"),
                line_string,
                record.args()
            )
        }
    }

    fn flush(&self) {}
}

pub fn init_logger(level: Level) -> Result<(), SetLoggerError> {
    let init_time = Instant::now();
    let logger = ScrubLog { init_time };
    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(level.to_level_filter());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smoke() {
        let _ = init_logger(log::Level::Trace);
        trace!("A");
        debug!("B");
        info!("C");
        warn!("D");
        error!("E");
    }
}
