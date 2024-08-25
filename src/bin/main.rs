use crate::prelude::*;

use rs_cli_template::{logger::setup_logging, prelude};
use tracing::{debug, info};

fn main() {
    // Initialise the global logger
    setup_logging(4);
    info!("starting program");
    debug!("welcome to hell");
}
