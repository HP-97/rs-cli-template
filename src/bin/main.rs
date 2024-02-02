use crate::prelude::*;

use rs_cli_template::{
    cli::parse_args,
    prelude,
    utils::{config::AppConfig, logger},
};
use std::{io::IsTerminal, process::exit, str::FromStr};
use tracing::Level;

fn main() -> Result<()>{
    let m = parse_args();
    let cfg = match AppConfig::new(&m) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "failed to create application configuration. REASON: {}",
                e.to_string()
            );
            exit(1)
        }
    };

    if cfg.log_level > 0 {
        let log_level = match tracing::Level::from_str(&cfg.log_level.to_string()) {
            Ok(v) => v,
            Err(_) => {
                eprintln!("invalid tracing level = {}", &cfg.log_level);
                exit(1)
            }
        };
        if let Err(e) = logger::setup_logging(log_level) {
            eprintln!("failed to setup logging. REASON: {}", e.to_string());
            exit(1)
        };
    }

    tracing::event!(Level::DEBUG, "program START");

    // Below is an code example of working with stdin
    if !std::io::stdin().is_terminal() {
        let mut buf = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut buf) {
            tracing::error!("{}", e.to_string());
        };
        println!("STDIN: {}", buf);
    }
    Ok(())
}
