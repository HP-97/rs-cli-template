use dotenv::dotenv;
use tracing::{event, Level};

use crate::{config::Config, logging::init_logging};

mod cli;
mod config;
mod utils;
mod logging;

fn main() {
    // Read .env environmental variables
    dotenv().ok();
    // Init logging
    init_logging();
    event!(Level::INFO, "hello world!");
    let cfg = Config::new();
    println!("{:#?}", cfg);
}
