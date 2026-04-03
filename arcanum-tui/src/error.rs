use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArcanumError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Seer error: {0}")]
    Seer(#[from] seer_core::SeerError),

    #[error("Tome error: {0}")]
    Tome(#[from] tome_core::TomeError),

    #[error("Terminal error: {0}")]
    Terminal(String),
}

pub type Result<T> = std::result::Result<T, ArcanumError>;
