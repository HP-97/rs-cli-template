use crate::prelude::*;

use config::{builder::DefaultState, Config, ConfigBuilder, Environment, File};
use serde::{Deserialize, Serialize};

use crate::cli::Cli;

const DEFAULT_CONFIG_NAME: &str = "config.toml";

const ENV_PREFIX: &str = "app";

#[derive(Debug, Serialize, Deserialize)]
/// All applications related configuration should go here
pub struct AppConfig {
    /// Determines output logging level
    pub log_level: usize,
}

impl AppConfig {
    pub fn new(cli_args: Option<&Cli>) -> Result<Self> {
        let log_level = match cli_args {
            Some(args) => match args.debug {
                0 => 1,
                // this sets -v to log level INFO, -vv = DEBUG, -vvv = TRACE
                0..=3 => args.debug + 2,
                level if level > 3 => 5,
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
        match s.build()?.try_deserialize::<AppConfig>() {
            Ok(v) => Ok(v),
            Err(e) => return Err(AppError::Config(e)),
        }
    }
}
