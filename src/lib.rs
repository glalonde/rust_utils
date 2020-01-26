use ansi_term::Color;
use log::{Level, Log, Metadata, Record, SetLoggerError};
use std::borrow::Borrow;
use std::fmt;
use std::time::{Duration, Instant};
// Duration formatting
#[derive(Clone, Copy, Debug)]
pub struct TimeFormat<T: Borrow<Duration>>(pub T);
impl<T: Borrow<Duration>> fmt::Display for TimeFormat<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let dur: &Duration = self.0.borrow();
        // The format string pads the fractional part out to 3 decimals.
        if dur.as_secs() > 0 {
            write!(f, "{}.{:0>3}s", dur.as_secs(), dur.subsec_millis(),)
        } else if dur.subsec_millis() > 0 {
            let integral = dur.subsec_millis();
            write!(
                f,
                "{}.{:0>3}ms",
                integral,
                dur.subsec_micros() - integral * 1000
            )
        } else if dur.subsec_micros() > 0 {
            let integral = dur.subsec_micros();
            write!(
                f,
                "{}.{:0>3}µs",
                integral,
                dur.subsec_nanos() - integral * 1000
            )
        } else {
            write!(f, "{:6}ns", dur.subsec_nanos())
        }
    }
}

struct ColorLevel(Level);
impl fmt::Display for ColorLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Level::Trace => Color::Purple.paint("TRACE"),
            Level::Debug => Color::Blue.paint("DEBUG"),
            Level::Info => Color::Green.paint("INFO "),
            Level::Warn => Color::Yellow.paint("WARN "),
            Level::Error => Color::Red.paint("ERROR"),
        }
        .fmt(f)
    }
}

struct ScrubLog {
    init_time: Instant,
    filter: env_logger::filter::Filter,
}
impl ScrubLog {
    fn from_filter(filter_string: &str) -> ScrubLog {
        use env_logger::filter::Builder;
        let mut builder = Builder::new();
        if !filter_string.is_empty() {
            builder.parse(filter_string);
        }
        ScrubLog {
            init_time: Instant::now(),
            filter: builder.build(),
        }
    }
}

impl Log for ScrubLog {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.filter.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        if !self.filter.matches(record) {
            return;
        }
        let line_string = match record.line() {
            Some(x) => format!("{}", x),
            None => String::from("-"),
        };

        let timestamp = Instant::now() - self.init_time;
        println!(
            "{}]{} [{}:{}] {}",
            ColorLevel(record.level()),
            TimeFormat(timestamp),
            record.module_path().unwrap_or("-"),
            line_string,
            record.args()
        )
    }

    fn flush(&self) {}
}

pub fn init_with_filter_string(filter_string: &str) -> Result<(), SetLoggerError> {
    let logger = ScrubLog::from_filter(filter_string);
    let log_level = logger.filter.filter();
    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(log_level);
    Ok(())
}

pub fn init() -> Result<(), SetLoggerError> {
    init_with_filter_string("")
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::{debug, error, info, trace, warn};
    #[test]
    fn smoke() {
        init().unwrap();
        trace!("A");
        debug!("B");
        info!("C");
        warn!("D");
        error!("E");
    }
}
