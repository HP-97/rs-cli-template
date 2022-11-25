use tracing::{event, Level};

use crate::{config::Config, logging::init_logging};

mod cli;
mod config;
mod utils;
mod logging;

fn main() {

    // Init config struct
    let config = Config::new();
    // Initialise global logging
    init_logging();
    event!(Level::INFO, "hello world!");
    // let cfg = Config::new();
    println!("{:#?}", config);
}
