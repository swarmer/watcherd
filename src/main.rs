extern crate watcherd;

use std::process::exit;

use watcherd::args;
use watcherd::config;
use watcherd::watcher;
use watcherd::logging;


fn run_watcher(config_path: &str) -> i32 {
    let config;
    match config::read_config(&config_path) {
        Ok(c) => config = c,
        Err(err) => {
            println!("Error: {}", err);
            return 1;
        },
    }

    match watcher::run(config) {
        Ok(()) => {},
        Err(err) => {
            println!("Error: {}", err);
            return 1;
        },
    }

    return 0;
}


fn dispatch_command() -> i32 {
    logging::init_logging();

    match args::parse_command() {
        args::WatcherdCommand::Version => {
            println!("{}", watcherd::VERSION);
            return 0;
        },
        args::WatcherdCommand::Run { config_path } => {
            return run_watcher(&config_path);
        },
    }
}


fn main() {
    exit(dispatch_command());
}
