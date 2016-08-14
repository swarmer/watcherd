extern crate watcherd;
use watcherd::args;


fn main() {
    match args::parse_args() {
        args::WatcherdArgs { flag_version: true, .. } => {
            println!("{}", watcherd::VERSION);
        },
        _ => {
            println!("{}", "wat");
        }
    }
}
