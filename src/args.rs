use docopt;


const USAGE: &'static str = "
Process supervisor

Usage:
  watcherd run [(-c <config-path> | --config <config-path>)]
  watcherd --version
  watcherd (-h | --help)

Options:
  -c --config <config-path>  Path to the config file [default: watcherd.json]
  --version  Show version.
  -h --help  Show this screen.
";


#[derive(Debug, RustcDecodable)]
struct DocoptWatcherdArgs {
    cmd_run: bool,
    flag_version: bool,
    flag_config: String,
}


#[derive(Debug)]
pub enum WatcherdCommand {
    Version,
    Run { config_path: String },
}


fn parse_args() -> DocoptWatcherdArgs {
    docopt::Docopt::new(USAGE)
    .and_then(|docopt| docopt.decode())
    .unwrap_or_else(|e| e.exit())
}

pub fn parse_command() -> WatcherdCommand {
    match parse_args() {
        DocoptWatcherdArgs { flag_version: true, .. } => WatcherdCommand::Version,
        DocoptWatcherdArgs { cmd_run: true, flag_config: config, .. } =>
            WatcherdCommand::Run { config_path: config },
        _ => {
            unreachable!();
        }
    }
}
