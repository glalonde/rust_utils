use log::{debug, error, info, trace, warn};

fn main() {
    scrub_log::init().unwrap();
    trace!("Lorem ipsum");
    debug!("dolor sit amet,");
    info!("consectetur adipiscing elit,");
    warn!("ed do eiusmod");
    error!("tempor incididunt");
}
