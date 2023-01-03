use anyhow::{Result};
use config::{Config as ConfigRS, ConfigError, Environment, File, ConfigBuilder, builder::DefaultState};
use serde_derive::{Deserialize, Serialize};

const DEFAULT_CONFIG_NAME: &str = "config.toml";

const ENV_PREFIX: &str = "APP";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // Determines output logging level
    pub log_level: String,
}

impl Config {
    pub fn new() -> Result<Self,ConfigError> {
        // let matches = parse_args();
        // Override environment variables as required

        // Get the default config path 
        let s: ConfigBuilder<DefaultState>;
        s = ConfigRS::builder()
            .add_source(File::with_name(DEFAULT_CONFIG_NAME).required(false))
            .add_source(Environment::with_prefix(ENV_PREFIX))
            // NOTE: Define defaults here
            .set_default("log_level", "error")?;

        // Build the config
        s.build()?.try_deserialize()
    }
}