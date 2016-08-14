extern crate rustc_serialize;
extern crate docopt;
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
struct WatcherdArgs {
    cmd_run: bool,
    flag_version: bool,
    flag_config: String,
}


fn parse_args() -> WatcherdArgs {
    let args: WatcherdArgs =
        Docopt::new(USAGE)
        .and_then(|docopt| docopt.decode())
        .unwrap_or_else(|e| e.exit());

    args
}


fn main() {
    let args = parse_args();
    println!("{:?}", args);
}
