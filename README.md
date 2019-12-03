# scrub_log

[![Build Status][bi]][bl] [![Crates.io][ci]][cl] ![MIT][li]

[bi]: https://travis-ci.org/glalonde/scrub_log.svg?branch=master 
[bl]: https://travis-ci.org/glalonde/scrub_log

[ci]: https://img.shields.io/crates/v/scrub_log.svg
[cl]: https://crates.io/crates/scrub_log/

[li]: https://img.shields.io/badge/license-MIT%2FApache-blue.svg

A log formatter that has a default with colors, filename, line number, and program runtime.

## Usage

Add this crate to `Cargo.toml`

```toml
[dependencies]
scrub_log = "0.1.0"
```

Now you can easily print a nice log line for all your printf debugging needs, glog style.

```rust
use log::{debug, error, info, trace, warn};

fn main() {
    scrub_log::init().unwrap();
    trace!("Lorem ipsum");
    debug!("dolor sit amet,");
    info!("consectetur adipiscing elit,");
    warn!("ed do eiusmod");
    error!("tempor incididunt");
}
```
Example output:
```
TRACE 7.761µs [log_example:5] Lorem ipsum
DEBUG 44.51µs [log_example:6] dolor sit amet,
INFO 58.178µs [log_example:7] consectetur adipiscing elit,
WARNING 68.099µs [log_example:8] ed do eiusmod
ERROR 77.319µs [log_example:9] tempor incididunt
```
