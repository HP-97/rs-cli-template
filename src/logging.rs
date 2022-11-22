use std::str::FromStr;

use anyhow::Result;
use serde::{Serialize, Deserialize};
use tracing::{event, Level};


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
                event!(Level::ERROR, "invalid log level: {}. Defaulting to ERROR", s);
                LogLevel::ERROR
            }
        };
        Ok(res)
    }
}

// impl TryFrom<u8> for LogLevel {
//     type Error = ();

//     fn try_from(v: u8) -> Result<Self, Self::Error> {
//         match v {
//             0 => Ok(LogLevel::ERROR),
//             1 => Ok(LogLevel::INFO),
//             2 => Ok(LogLevel::DEBUG),
//             3 => Ok(LogLevel::TRACE),
//             _ => Err(())
//         }
//     }
// } 


// TODO: Complete to init global logger
// TODO: Add functionality to configure a log file that is on default placed in ./log/ where . is the directory of the binary.
pub fn init_logging() {
    tracing_subscriber::fmt::init();
        // .with_writer();
}