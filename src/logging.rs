use std::str::FromStr;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{event, Level};
use tracing_subscriber::{
    fmt::{writer::MakeWriterExt, Layer, time},
    prelude::__tracing_subscriber_SubscriberExt,
};

use crate::utils::get_exe_parent_path;

#[derive(Debug, Serialize, Deserialize)]
pub enum LogLevel {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

impl FromStr for LogLevel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s.to_lowercase().as_str() {
            "trace" => LogLevel::TRACE,
            "debug" => LogLevel::DEBUG,
            "info" => LogLevel::INFO,
            "warn" => LogLevel::WARN,
            "error" => LogLevel::ERROR,
            // Default log level
            _ => {
                event!(
                    Level::ERROR,
                    "invalid log level: {}. Defaulting to ERROR",
                    s
                );
                LogLevel::ERROR
            }
        };
        Ok(res)
    }
}


// TODO: Complete to init global logger
// TODO: Add functionality to configure a log file that is on default placed in ./log/ where . is the directory of the binary.
// Documentation for tracing_appender: https://docs.rs/tracing-appender/latest/tracing_appender/
pub fn init_logging() {
    // FIXME Unhandled Result
    let logfile_dir = format!("{}/{}", get_exe_parent_path().unwrap().display(), "log");
    println!("{logfile_dir}");
    let logfile = tracing_appender::rolling::daily(logfile_dir, "template-logs");
    let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);
    let subscriber = tracing_subscriber::registry()
        .with(Layer::new().with_writer(logfile).with_ansi(false))
        .with(Layer::new().with_writer(stdout).with_timer(time::));
    // tracing_subscriber::registry().with(subscriber).init();
    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to set global logging instance")
}
