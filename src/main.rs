extern crate watcherd;
use watcherd::args;


fn main() {
    match args::parse_command() {
        args::WatcherdCommand::Version => {
            println!("{}", watcherd::VERSION);
        },
        args::WatcherdCommand::Run { config_path } => {
            println!("Running from {}", config_path);
        },
    }
}
