use std::path::PathBuf;

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
    /// If true, do not do any actual work
    pub dry_run: bool,
}

impl AppConfig {
    pub fn new(cli_args: &Cli) -> Result<Self> {
        let log_level = match cli_args.debug {
            0 => 1,
            0..=3 => cli_args.debug + 2,
            level if level > 3 => 5,
            _ => unreachable!(),
        };
        // Override environment variables as required

        // Get the default config path
        let s: ConfigBuilder<DefaultState>;
        s = Config::builder()
            .add_source(File::with_name(DEFAULT_CONFIG_NAME).required(false))
            // e.g. `APP_USER=alice ./target/app` would set the 'user' key
            .add_source(Environment::with_prefix(ENV_PREFIX))
            // NOTE: Define defaults here
            .set_default("log_level", log_level)?
            .set_default("dry_run", false)?;

        // Build the config
        match s.build()?.try_deserialize::<AppConfig>() {
            Ok(v) => Ok(v),
            Err(e) => return Err(AppError::Config(e)),
        }
    }
}
