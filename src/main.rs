extern crate watcherd;


fn main() {
    let args = watcherd::args::parse_args();
    println!("{:?}", args);
}
