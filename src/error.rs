
use config::ConfigError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError{
    #[error(transparent)]
    ConfigError(#[from] ConfigError), 
}
