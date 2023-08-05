use crate::prelude::*;

use rs_cli_template::{
    cli::parse_args,
    prelude,
    utils::{config::AppConfig, logger},
};
use std::{process::exit, str::FromStr};
use tracing::Level;

fn main() -> Result<()> {
    let m = parse_args();
    let cfg = AppConfig::new(Some(&m))?;

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
    Ok(())
}
