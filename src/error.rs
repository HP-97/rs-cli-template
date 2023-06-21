use config::ConfigError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    /// For starter, to remove as code matures
    #[error("generic error: {0}")]
    Generic(String),
    #[error(transparent)]
    Config(#[from] ConfigError),
}
