pub mod config;
pub mod service;

pub use config::AppConfig;
pub use service::AppService;

use thiserror::Error;

/// Unified application error type.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("BPA interaction error: {0}")]
    Bpa(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
