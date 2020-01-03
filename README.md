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
scrub_log = "0.1.1"
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
TRACE]15.464µs [log_example:10] Lorem ipsum
DEBUG]42.350µs [log_example:11] dolor sit amet,
INFO ]45.667µs [log_example:12] consectetur adipiscing elit,
WARN ]48.431µs [log_example:13] ed do eiusmod
ERROR]50.754µs [log_example:14] tempor incididunt
```
![Image of text](https://glalon.de/content/images/2020/01/scrub_log_screen.png)

## Filters
This library imports the filter functionality of [`env_logger`](https://crates.io/crates/env_logger), so it is easy to add a filter string in that same format.

One convenient way to do so is with a command line flag rather than an environment variable. For example, using the [`gflags`](https://crates.io/crates/gflags) library:
```
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
```
This allows one to request different log levels on demand, with a default:
```
$ cargo run --example log_example -- --log_filter warn
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/examples/log_example --log_filter warn`
WARN ]11.740µs [log_example:13] ed do eiusmod
ERROR]44.914µs [log_example:14] tempor incididunt
$ cargo run --example log_example -- --log_filter log_example=debug
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/examples/log_example --log_filter log_example=debug`
DEBUG]14.542µs [log_example:11] dolor sit amet,
INFO ]42.239µs [log_example:12] consectetur adipiscing elit,
WARN ]45.890µs [log_example:13] ed do eiusmod
ERROR]48.758µs [log_example:14] tempor incididunt
```

See the `env_logger` docs for exact specifications for the filter string, but generally it is either `$global_log_level` or `$module1_name=$module1_log_level,$module2_name=$module2_log_level`.

The default log level, inherited from `env_logger`, is `warn`. This can be changed by specifying a filter string e.g:
```
scrub_log::init_with_filter_string("trace").unwrap();
```