use rs_cli_template::{
    cli::parse_args,
    error::AppError,
    utils::{config::AppConfig, logger},
};
use std::{process::exit, str::FromStr};

fn main() -> Result<(), AppError> {
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

    println!("{:?}", m);
    println!("{:?}", cfg);
    Ok(())
}
