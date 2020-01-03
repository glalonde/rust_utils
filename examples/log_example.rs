use log::{debug, error, info, trace, warn};

gflags::define! {
    --log_filter: &str = "log_example=info"
}

fn main() {
    gflags::parse();
    scrub_log::init_with_filter_string(LOG_FILTER.flag).unwrap();
    trace!("Lorem ipsum");
    debug!("dolor sit amet,");
    info!("consectetur adipiscing elit,");
    warn!("ed do eiusmod");
    error!("tempor incididunt");
}
