
#[macro_use]
extern crate log;
extern crate rust_utils;

fn main() {
    let _ = rust_utils::init_logger(log::Level::Trace);
    trace!("How much wood");
    debug!("would a woodchuck chuck");
    info!("if a woodchuck");
    warn!("could chuck");
    error!("wood?");
}