use rs_cli_template::{clap::parse_args};

fn main() {
    let m = parse_args();
    println!("{:?}", m);
}