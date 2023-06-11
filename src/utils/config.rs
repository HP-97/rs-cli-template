use config::{Config, ConfigError, Environment, File, ConfigBuilder, builder::DefaultState};
use serde::{Serialize, Deserialize};

use crate::cli::Cli;

const DEFAULT_CONFIG_NAME: &str = "config.toml";

const ENV_PREFIX: &str = "app";

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    // Determines output logging level
    pub log_level: usize,
}

impl AppConfig {
    pub fn new(cli_args: Option<&Cli>) -> Result<Self,ConfigError> {
        let log_level = match cli_args {
            Some(args) => match args.debug {
                0 => 1,
                0 ..= 5 => args.debug,
                level if level > 5 => 5,
                _ => unreachable!(),
            },
            None => 0,
        };
        // Override environment variables as required

        // Get the default config path 
        let s: ConfigBuilder<DefaultState>;
        s = Config::builder()
            .add_source(File::with_name(DEFAULT_CONFIG_NAME).required(false))
            // e.g. `APP_USER=alice ./target/app` would set the 'user' key
            .add_source(Environment::with_prefix(ENV_PREFIX))
            // NOTE: Define defaults here
            .set_default("log_level", log_level)?;

        // Build the config
        s.build()?.try_deserialize()
    }
}