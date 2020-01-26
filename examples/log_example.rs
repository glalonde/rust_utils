use log::{debug, error, info, trace, warn};
use std::{thread, time};

gflags::define! {
    --log_filter: &str = "log_example=trace"
}

fn main() {
    gflags::parse();
    scrub_log::init_with_filter_string(LOG_FILTER.flag).unwrap();
    trace!("Lorem ipsum");
    debug!("dolor sit amet,");
    info!("consectetur adipiscing elit,");
    warn!("ed do eiusmod");
    error!("tempor incididunt");

    loop {
        thread::sleep(time::Duration::from_millis(15));
        trace!("reporting live");
        debug!("reporting live");
        info!("reporting live");
        warn!("reporting live");
        error!("reporting live");
    }
}
