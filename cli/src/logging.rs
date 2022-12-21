use std::str::FromStr;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{event, Level};
use tracing_subscriber::{
    fmt::{time, writer::MakeWriterExt, Layer},
    prelude::__tracing_subscriber_SubscriberExt,
};

use crate::{utils::get_exe_parent_path, cli::parse_args};

// init_logging configures a log file that is on default placed in ./logs/ where . is the directory of the binary.
// Documentation for tracing_appender: https://docs.rs/tracing-appender/latest/tracing_appender/
pub fn init_logging() -> Result<()>{
    // Create a PathBuf that points to parent path of executable + '/logs'
    let mut logfile_dir = get_exe_parent_path()?;
    logfile_dir.push("logs");
    let logfile_dir = logfile_dir.display().to_string();

    // Get the max logging level
    // It is technically possible to dynamically reload the max log level (Original plan was to log level at a default and update
    // according to config struct). Can be done if needed
    // Reference: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/reload/index.html
    let matches = parse_args();
    let log_level = match matches.get_one::<u8>("verbose") {
        Some(1) => Level::INFO,
        Some(2) => Level::DEBUG,
        Some(&x) if x >= 3 => Level::TRACE,
        // Default logging level
        _ => Level::ERROR
    };

    let logfile = tracing_appender::rolling::daily(logfile_dir, "template-logs").with_max_level(log_level);
    let stdout = std::io::stdout.with_max_level(log_level);

    let subscriber = tracing_subscriber::registry()
        .with(
            Layer::new()
                .with_writer(logfile)
                .with_ansi(false)
                .with_timer(time::LocalTime::rfc_3339()),
        )
        .with(
            Layer::new()
                .with_writer(stdout)
                .with_timer(time::LocalTime::rfc_3339()),
        );

    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to set global logging instance");
    
    Ok(())
}