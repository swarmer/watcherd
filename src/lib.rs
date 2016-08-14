extern crate rustc_serialize;
extern crate docopt;
extern crate serde;
extern crate serde_json;
extern crate nix;


pub mod args;
pub mod config;
pub mod watcher;


pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
