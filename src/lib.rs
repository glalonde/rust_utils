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
        let round_3_decimals = |x: f64| (1000. * x).round() / 1000.;
        if dur.as_secs() > 0 {
            write!(f, "{}s", round_3_decimals(dur.as_secs_f64()))
        } else if dur.subsec_millis() > 0 {
            write!(f, "{}ms", round_3_decimals(dur.as_secs_f64() * 1000.0))
        } else if dur.subsec_micros() > 0 {
            write!(f, "{}µs", round_3_decimals(dur.as_secs_f64() * 1000000.0))
        } else {
            write!(f, "{}ns", dur.subsec_nanos())
        }
    }
}

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
                TimeFormat(timestamp),
                record.module_path().unwrap_or("-"),
                line_string,
                record.args()
            )
        }
    }

    fn flush(&self) {}
}

pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    let init_time = Instant::now();
    let logger = ScrubLog { init_time };
    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(level.to_level_filter());
    Ok(())
}

pub fn init() -> Result<(), SetLoggerError> {
    init_with_level(log::Level::Trace)
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
