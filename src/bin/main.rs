use rs_cli_template::{cli::parse_args, utils::config::AppConfig};

fn main() {
    let m = parse_args();

    let c = AppConfig::new(None);
    println!("{:?}", m);
    println!("{:?}", c);
}