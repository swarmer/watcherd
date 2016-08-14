#[macro_use]
extern crate log;

extern crate rustc_serialize;
extern crate docopt;
extern crate serde;
extern crate serde_json;
extern crate nix;
extern crate env_logger;


pub mod args;
pub mod config;
pub mod watcher;
pub mod logging;


pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
