extern crate watcherd;
use watcherd::args;
use watcherd::config;


fn main() {
    match args::parse_command() {
        args::WatcherdCommand::Version => {
            println!("{}", watcherd::VERSION);
        },
        args::WatcherdCommand::Run { config_path } => {
            match config::read_config(&config_path) {
                Ok(config) => println!("Config: {:?}", config),
                Err(err) => println!("Error: {}", err),
            }
        },
    }
}
