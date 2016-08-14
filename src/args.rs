use docopt::Docopt;


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
pub struct WatcherdArgs {
    pub cmd_run: bool,
    pub flag_version: bool,
    pub flag_config: String,
}


pub fn parse_args() -> WatcherdArgs {
    let args =
        Docopt::new(USAGE)
        .and_then(|docopt| docopt.decode())
        .unwrap_or_else(|e| e.exit());

    args
}
