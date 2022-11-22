use std::env;

use anyhow::Result;
use config::{Config as ConfigRS, ConfigError, Environment, File, ConfigBuilder, builder::DefaultState};
use serde_derive::{Deserialize, Serialize};

use crate::cli::parse_args;

// TODO: Should make the name of the default configuration file constant
// const default_config_file_path

const ENV_PREFIX: &str = "APP";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // Determines output logging level
    pub log_level: String,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let matches = parse_args();
        let s: ConfigBuilder<DefaultState>;
        s = ConfigRS::builder()
            .add_source(File::with_name("./config.toml").required(false))
            .add_source(Environment::with_prefix(ENV_PREFIX));

        // log_level CLI
        let log_level = match matches.get_one::<u8>("verbose") {
            Some(1) => "info",
            Some(2) => "debug",
            Some(3) => "trace",
            _ => ""
        };
        if !log_level.is_empty() {
            env::set_var(format!("{}_LOG_LEVEL", ENV_PREFIX), log_level);
        }
        
        // Build the config
        s.build()?.try_deserialize()
    }
}
