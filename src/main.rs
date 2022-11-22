use crate::config::Config;

mod cli;
mod config;
mod utils;
mod logging;

fn main() {
    let cfg = Config::new();
    println!("{:#?}", cfg);
}
