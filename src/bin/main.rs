use crate::prelude::*;

use rs_cli_template::{
    cli::parse_args,
    utils::{config::AppConfig, logger},
    prelude,
};
use tracing::Level;
use std::{process::exit, str::FromStr, io::IsTerminal};

fn main() -> Result<()> {
    let m = parse_args();
    let cfg = AppConfig::new(&m)?;

    if cfg.log_level > 0 {
        let log_level = match tracing::Level::from_str(&cfg.log_level.to_string()) {
            Ok(v) => v,
            Err(_) => {
                eprintln!("invalid tracing level = {}", &cfg.log_level);
                exit(1)
            }
        };
        logger::setup_logging(log_level)?;
    }

    tracing::event!(Level::DEBUG, "program START");

    // Below is an code example of working with stdin
    if ! std::io::stdin().is_terminal() {
        let mut buf = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut buf) {
            tracing::error!("{}", e.to_string());
        };
        println!("STDIN: {}", buf);
    }
    Ok(())
}
