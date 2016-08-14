extern crate rustc_serialize;
extern crate docopt;
extern crate serde;
extern crate serde_json;

pub mod args;
pub mod config;


pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
