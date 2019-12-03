use log::{debug, error, info, trace, warn};

fn main() {
    scrub_log::init().unwrap();
    trace!("How much wood");
    debug!("would a woodchuck chuck");
    info!("if a woodchuck");
    warn!("could chuck");
    error!("wood?");
}
